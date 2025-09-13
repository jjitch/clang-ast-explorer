use crate::interface::{AstEntityFull, AstEntityLite};

#[derive(Debug)]
pub enum ClangEngineError {
    InitializationError(String),
}

type EngineCallResult<R> = Result<R, ClangEngineError>;

#[derive(Debug)]
pub struct Diagnostic {
    pub severity: String,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

pub enum Msg {
    ParseSourceCode(
        tokio::sync::oneshot::Sender<EngineCallResult<AstEntityLite>>,
        std::path::PathBuf,
    ),
    RevealEntity(
        tokio::sync::oneshot::Sender<EngineCallResult<AstEntityFull>>,
        String,
    ),
}

struct ClangReceiver;

impl ClangReceiver {
    fn receive(rx: &std::sync::mpsc::Receiver<Msg>) -> Result<(), ClangEngineError> {
        let clang = clang::Clang::new().map_err(ClangEngineError::InitializationError)?;
        loop {
            let index = clang::Index::new(&clang, false, true);
            let mut entity_store = std::collections::HashMap::<String, clang::Entity<'_>>::new();
            match rx.recv() {
                Ok(Msg::ParseSourceCode(sender, path)) => {
                    let tu = index
                        .parser(&path)
                        .arguments(&["-std=c++17"])
                        .parse()
                        .map_err(|e| ClangEngineError::InitializationError(format!("{:?}", e)))?;
                    let entity = tu.get_entity();
                    let new_id = uuid::Uuid::new_v4().to_string();
                    entity_store.insert(new_id.clone(), entity);
                    sender
                        .send(Ok(AstEntityLite {
                            id: new_id,
                            kind: format!("{:?}", entity.get_kind()),
                        }))
                        .map_err(|e| {
                            ClangEngineError::InitializationError(format!(
                                "Failed to send entity ID: {:?}",
                                e
                            ))
                        })?;
                }
                Ok(Msg::RevealEntity(sender, entity_id)) => {
                    if let Some(entity) = entity_store.get(&entity_id) {
                        println!("Revealing entity: {:?}", entity);
                        // Implement logic to reveal the entity in the UI if needed
                        let children = entity
                            .get_children()
                            .iter()
                            .map(|child| AstEntityLite {
                                id: uuid::Uuid::new_v4().to_string(),
                                kind: format!("{:?}", child.get_kind()),
                            })
                            .collect::<Vec<_>>();
                        sender
                            .send(Ok(AstEntityFull {
                                properties: vec![],
                                children,
                            }))
                            .map_err(|e| {
                                ClangEngineError::InitializationError(format!(
                                    "Failed to send reveal confirmation: {:?}",
                                    e
                                ))
                            })?;
                    } else {
                        sender
                            .send(Err(ClangEngineError::InitializationError(
                                "Entity not found".into(),
                            )))
                            .map_err(|e| {
                                ClangEngineError::InitializationError(format!(
                                    "Failed to send error: {:?}",
                                    e
                                ))
                            })?;
                    }
                }
                Err(_) => {}
            }
        }
    }
}

pub struct EngineHandle(std::sync::mpsc::Sender<Msg>);

impl EngineHandle {
    pub async fn call<F, R>(&self, f: F) -> EngineCallResult<R>
    where
        F: FnOnce(tokio::sync::oneshot::Sender<EngineCallResult<R>>) -> Msg + Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel::<EngineCallResult<R>>();
        self.0.send(f(tx)).unwrap();
        rx.await.unwrap()
    }
}

pub fn spawn_engine() -> EngineHandle {
    let (tx, rx) = std::sync::mpsc::channel::<Msg>();
    std::thread::spawn(move || {
        while let Err(e) = ClangReceiver::receive(&rx) {
            eprintln!("Error receiving message: {:?}", e);
        }
    });
    EngineHandle(tx)
}
