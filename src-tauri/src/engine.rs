use crate::interface::{AstEntityFull, AstEntityLite};

#[derive(Debug)]
pub enum ClangEngineError {
    InitializationError(String),
    PreviousTranslationUnitExist,
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
    AbortTranslationUnit(tokio::sync::oneshot::Sender<EngineCallResult<()>>),
}

struct TranslationUnitSession<'i, 'tu> {
    entity_store: std::collections::HashMap<String, clang::Entity<'i>>,
    tu: &'tu clang::TranslationUnit<'i>,
}

impl TranslationUnitSession<'_, '_> {
    fn receive(&mut self, rx: &std::sync::mpsc::Receiver<Msg>) -> Result<(), ClangEngineError> {
        loop {
            match rx.recv() {
                Ok(Msg::ParseSourceCode(sender, _)) => {
                    sender
                        .send(Err(ClangEngineError::PreviousTranslationUnitExist))
                        .map_err(|e| {
                            ClangEngineError::InitializationError(format!(
                                "Failed to send error: {:?}",
                                e
                            ))
                        })?;
                }
                Ok(Msg::RevealEntity(sender, entity_id)) => {
                    if let Some(entity) = self.entity_store.get(&entity_id) {
                        println!("Revealing entity: {:?}", entity);
                        // Implement logic to reveal the entity in the UI if needed
                        let mut children = vec![];
                        for child in entity.get_children() {
                            let child_id = uuid::Uuid::new_v4().to_string();
                            self.entity_store.insert(child_id.clone(), child);
                            children.push(AstEntityLite {
                                id: child_id,
                                kind: format!("{:?}", child.get_kind()),
                            });
                        }
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
                Ok(Msg::AbortTranslationUnit(sender)) => {
                    sender.send(Ok(())).map_err(|e| {
                        ClangEngineError::InitializationError(format!(
                            "Failed to send abort confirmation: {:?}",
                            e
                        ))
                    })?;
                    break;
                }
                Err(_) => {}
            }
        }
        Ok(())
    }
}

struct ClangReceiver;

impl ClangReceiver {
    fn receive(rx: &std::sync::mpsc::Receiver<Msg>) -> Result<(), ClangEngineError> {
        let clang = clang::Clang::new().map_err(ClangEngineError::InitializationError)?;
        loop {
            match rx.recv() {
                Ok(Msg::ParseSourceCode(sender, path)) => {
                    let index = clang::Index::new(&clang, false, false);
                    let mut entity_store =
                        std::collections::HashMap::<String, clang::Entity<'_>>::new();
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
                    let mut session = TranslationUnitSession {
                        entity_store,
                        tu: &tu,
                    };
                    session.receive(rx)?;
                }
                Ok(Msg::RevealEntity(sender, _entity_id)) => {
                    sender
                        .send(Err(ClangEngineError::InitializationError(
                            "No active translation unit".into(),
                        )))
                        .map_err(|e| {
                            ClangEngineError::InitializationError(format!(
                                "Failed to send error: {:?}",
                                e
                            ))
                        })?;
                }
                Ok(Msg::AbortTranslationUnit(sender)) => {
                    sender.send(Ok(())).map_err(|e| {
                        ClangEngineError::InitializationError(format!(
                            "Failed to send abort confirmation: {:?}",
                            e
                        ))
                    })?;
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
