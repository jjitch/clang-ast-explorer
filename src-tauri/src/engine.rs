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

#[derive(Debug, Clone)]
pub struct AstEntityId(pub String);

pub enum Msg {
    ParseSourceCode(
        tokio::sync::oneshot::Sender<EngineCallResult<AstEntityId>>,
        std::path::PathBuf,
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
                Ok(msg) => match msg {
                    Msg::ParseSourceCode(sender, path) => {
                        let tu = index
                            .parser(&path)
                            .arguments(&["-std=c++17"])
                            .parse()
                            .map_err(|e| {
                                ClangEngineError::InitializationError(format!("{:?}", e))
                            })?;
                        let entity = tu.get_entity();
                        let new_id = uuid::Uuid::new_v4().to_string();
                        entity_store.insert(new_id.clone(), entity);
                        sender.send(Ok(AstEntityId(new_id))).map_err(|e| {
                            ClangEngineError::InitializationError(format!(
                                "Failed to send entity ID: {:?}",
                                e
                            ))
                        })?;
                    }
                },
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
