#[derive(Debug)]
pub enum ClangEngineError {
    InitializationError(String),
}

type EngineCallResult<R> = Result<R, ClangEngineError>;

pub enum Msg {
    CreateClang(tokio::sync::oneshot::Sender<EngineCallResult<()>>),
}

struct ClangReceiver;

impl ClangReceiver {
    fn receive(rx: &std::sync::mpsc::Receiver<Msg>) {
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    Msg::CreateClang(sender) => {
                        if let Ok(_c) = clang::Clang::new() {
                            sender.send(Ok(())).unwrap();
                            loop {
                                match rx.recv() {
                                    Ok(msg) => match msg {
                                        Msg::CreateClang(sender) => {
                                            sender
                                                .send(Err(ClangEngineError::InitializationError(
                                                    "Clang instance already created".into(),
                                                )))
                                                .unwrap();
                                        }
                                    },
                                    Err(_) => break,
                                }
                            }
                        } else {
                            sender
                                .send(Err(ClangEngineError::InitializationError(
                                    "Failed to create Clang instance".into(),
                                )))
                                .unwrap();
                        }
                    }
                },
                Err(_) => break,
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
        ClangReceiver::receive(&rx);
    });
    EngineHandle(tx)
}

#[tokio::test]
async fn test_clang_engine_initialization() {
    let engine = spawn_engine();
    let res = engine.call(|tx| Msg::CreateClang(tx)).await;
    assert!(res.is_ok());
}
