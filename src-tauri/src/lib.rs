pub mod command;
pub mod engine;
pub mod entity;

use tauri::Manager;

pub struct AppStateRaw {
    pub engine_handle: engine::EngineHandle,
}

pub type AppState = tokio::sync::Mutex<AppStateRaw>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState::new(AppStateRaw {
                engine_handle: engine::spawn_engine(),
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::parse_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
