#[derive(Debug)]
pub enum ClangEngineError {
    InitializationError(String),
    CreationError(String),
}

pub enum Msg {
    CreateClang(tokio::sync::oneshot::Sender<Result<(), ClangEngineError>>),
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
                                                .send(Err(ClangEngineError::CreationError(
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
                                .send(Err(ClangEngineError::CreationError(
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

pub fn spawn_engine() -> std::sync::mpsc::Sender<Msg> {
    let (tx, rx) = std::sync::mpsc::channel::<Msg>();
    std::thread::spawn(move || {
        ClangReceiver::receive(&rx);
    });
    tx
}

#[test]
fn test_clang_engine_initialization() {
    let tx = spawn_engine();

    let (sender, receiver) = tokio::sync::oneshot::channel();
    tx.send(Msg::CreateClang(sender)).unwrap();

    let res = receiver.blocking_recv();
    assert!(res.is_ok());
}
