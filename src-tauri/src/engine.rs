use crate::interface::{AstEntityFull, AstEntityLite};

#[derive(Debug)]
enum ClangMsgLoopError {
    // String field is deemed to be unused, but it's useful for debugging.
    // So we allow dead_code warning here.
    #[allow(dead_code)]
    SendingResponseFailed(String),
    InitializationError,
}

#[derive(Debug)]
pub enum ClangEngineError {
    PreviousTranslationUnitExist,
    EntityIdExpired,
    ParseSourceFailed { details: String },
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
    ExistTranslationUnit(tokio::sync::oneshot::Sender<EngineCallResult<bool>>),
    AbortTranslationUnit(tokio::sync::oneshot::Sender<EngineCallResult<()>>),
}

struct TranslationUnitSession<'tu> {
    entity_store: std::collections::HashMap<String, clang::Entity<'tu>>,
}

impl TranslationUnitSession<'_> {
    fn receive(&mut self, rx: &std::sync::mpsc::Receiver<Msg>) -> Result<(), ClangMsgLoopError> {
        loop {
            match rx.recv() {
                Ok(Msg::ParseSourceCode(sender, _)) => {
                    sender
                        .send(Err(ClangEngineError::PreviousTranslationUnitExist))
                        .or(Err(ClangMsgLoopError::SendingResponseFailed(
                            "ParseSourceCode".into(),
                        )))?;
                }
                Ok(Msg::RevealEntity(sender, entity_id)) => {
                    if let Some(entity) = self.entity_store.get(&entity_id) {
                        // Implement logic to reveal the entity in the UI if needed
                        let mut properties = vec![];
                        // Aquiring properties starts, but this is just a trial and subject to change.
                        if let Some(name) = entity.get_name() {
                            properties.push((String::from("name"), name));
                        }
                        if let Some(ty) = entity.get_type() {
                            properties
                                .push((String::from("Type>display_name"), ty.get_display_name()));
                        }

                        let source_range = {
                            if let Some(range) = entity.get_range() {
                                let start = range.get_start();
                                let end = range.get_end();
                                Some(crate::interface::SourceRange {
                                    start_line: start.get_file_location().line as i64,
                                    start_column: start.get_file_location().column as i64,
                                    end_line: end.get_file_location().line as i64,
                                    end_column: end.get_file_location().column as i64,
                                })
                            } else {
                                None
                            }
                        };

                        let mut children = vec![];
                        for child in entity.get_children() {
                            let child_id = uuid::Uuid::new_v4().to_string();
                            self.entity_store.insert(child_id.clone(), child);
                            children.push(AstEntityLite {
                                id: child_id,
                                kind: format!("{:?}", child.get_kind()),
                                display_name: child.get_display_name().unwrap_or_default(),
                            });
                        }
                        sender.send(Ok(AstEntityFull {
                            properties,
                            children,
                            source_range,
                        }))
                    } else {
                        sender.send(Err(ClangEngineError::EntityIdExpired))
                    }
                    .or(Err(ClangMsgLoopError::SendingResponseFailed(
                        format!("RevealEntity: {:?}", entity_id),
                    )))?;
                }
                Ok(Msg::ExistTranslationUnit(sender)) => {
                    sender.send(Ok(true)).map_err(|e| {
                        ClangMsgLoopError::SendingResponseFailed(format!(
                            "Failed to send TU existence: {:?}",
                            e
                        ))
                    })?;
                }
                Ok(Msg::AbortTranslationUnit(sender)) => {
                    sender.send(Ok(())).map_err(|e| {
                        ClangMsgLoopError::SendingResponseFailed(format!(
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
    fn receive(rx: &std::sync::mpsc::Receiver<Msg>) -> Result<(), ClangMsgLoopError> {
        let clang = clang::Clang::new().or(Err(ClangMsgLoopError::InitializationError))?;
        loop {
            match rx.recv() {
                Ok(Msg::ParseSourceCode(sender, path)) => {
                    let index = clang::Index::new(&clang, false, false);
                    let tu = if let Ok(tu) = index.parser(&path).arguments(&["-std=c++17"]).parse()
                    {
                        tu
                    } else {
                        sender
                            .send(Err(ClangEngineError::ParseSourceFailed {
                                details: format!("Failed to parse source file: {path:?}"),
                            }))
                            .or(Err(ClangMsgLoopError::SendingResponseFailed(format!(
                                "Failed to send parse error: {path:?}"
                            ))))?;
                        continue;
                    };
                    let mut entity_store =
                        std::collections::HashMap::<String, clang::Entity<'_>>::new();
                    let entity = tu.get_entity();
                    let new_id = uuid::Uuid::new_v4().to_string();
                    entity_store.insert(new_id.clone(), entity);
                    sender
                        .send(Ok(AstEntityLite {
                            id: new_id,
                            kind: format!("{:?}", entity.get_kind()),
                            display_name: entity.get_display_name().unwrap_or_default(),
                        }))
                        .or(Err(ClangMsgLoopError::SendingResponseFailed(format!(
                            "Failed to send entity: {path:?}"
                        ))))?;
                    let mut session = TranslationUnitSession { entity_store };
                    session.receive(rx)?;
                }
                Ok(Msg::RevealEntity(sender, entity_id)) => {
                    sender.send(Err(ClangEngineError::EntityIdExpired)).or(Err(
                        ClangMsgLoopError::SendingResponseFailed(format!(
                            "RevealEntity without TU: {:?}",
                            entity_id
                        )),
                    ))?;
                }
                Ok(Msg::ExistTranslationUnit(sender)) => {
                    sender
                        .send(Ok(false))
                        .or(Err(ClangMsgLoopError::SendingResponseFailed(
                            "ExistTranslationUnit without TU".into(),
                        )))?;
                }
                Ok(Msg::AbortTranslationUnit(sender)) => {
                    sender
                        .send(Ok(()))
                        .or(Err(ClangMsgLoopError::SendingResponseFailed(
                            "AbortTranslationUnit without TU".into(),
                        )))?;
                    break;
                }
                Err(e) => {
                    return Err(ClangMsgLoopError::SendingResponseFailed(format!(
                        "Channel receive error: {e:?}"
                    )));
                }
            }
        }
        Ok(())
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
            log::error!("An error occurred in the clang engine message loop: {e:?}");
        }
    });
    EngineHandle(tx)
}
