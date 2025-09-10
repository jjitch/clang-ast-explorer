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
        tokio::sync::oneshot::Sender<EngineCallResult<Vec<Diagnostic>>>,
        std::path::PathBuf,
    ),
}

struct ClangReceiver;

impl ClangReceiver {
    fn receive(rx: &std::sync::mpsc::Receiver<Msg>) -> Result<(), ClangEngineError> {
        let clang = clang::Clang::new().map_err(ClangEngineError::InitializationError)?;
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    Msg::ParseSourceCode(sender, path) => {
                        let index = clang::Index::new(&clang, false, false);
                        let tu = index
                            .parser(&path)
                            .arguments(&["-std=c++17"])
                            .parse()
                            .map_err(|e| {
                                ClangEngineError::InitializationError(format!("{:?}", e))
                            })?;
                        let diagnostics: Vec<Diagnostic> = tu
                            .get_diagnostics()
                            .into_iter()
                            .map(|diag| {
                                let loc = diag.get_location().get_file_location();
                                Diagnostic {
                                    severity: format!("{:?}", diag.get_severity()),
                                    message: diag.get_text(),
                                    file: loc
                                        .file
                                        .unwrap()
                                        .get_path()
                                        .to_string_lossy()
                                        .to_string(),
                                    line: loc.line,
                                    column: loc.column,
                                }
                            })
                            .collect();
                        sender.send(Ok(diagnostics)).map_err(|e| {
                            ClangEngineError::InitializationError(format!(
                                "Failed to send diagnostics: {:?}",
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
