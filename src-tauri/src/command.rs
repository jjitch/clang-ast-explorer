use std::io::Write;
use tauri::{Emitter, Manager};

#[tauri::command]
pub async fn parse_source(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    source_code: String,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let temp_dir = app_handle.path().temp_dir().map_err(|e| e.to_string())?;
    let source_file = temp_dir.join("ast-explorer.cpp");
    let mut file = std::fs::File::create(&source_file).map_err(|e| e.to_string())?;
    file.write_all(source_code.as_bytes())
        .map_err(|e| e.to_string())?;
    let res = app_state
        .engine_handle
        .call(|tx| crate::engine::Msg::ParseSourceCode(tx, source_file))
        .await;
    match res {
        Ok(entity_lite) => {
            match app_handle.emit_to(
                tauri::EventTarget::WebviewWindow {
                    label: "main".into(),
                },
                "ast-ready",
                entity_lite,
            ) {
                Ok(_) => {
                    println!("Emitted AST ready event to window");
                }
                Err(e) => eprintln!("Error emitting AST ready event to window: {:?}", e),
            }
            Ok(())
        }
        Err(e) => Err(format!("Error parsing source code: {:?}", e)),
    }
}

#[tauri::command]
pub async fn reveal_entity(
    state: tauri::State<'_, crate::AppState>,
    entity_id: String,
) -> Result<crate::interface::AstEntityFull, String> {
    let app_state = state.lock().await;
    let entity = app_state
        .engine_handle
        .call(|tx| crate::engine::Msg::RevealEntity(tx, entity_id))
        .await
        .map_err(|e| format!("Error revealing entity: {:?}", e))?;
    Ok(entity)
}
