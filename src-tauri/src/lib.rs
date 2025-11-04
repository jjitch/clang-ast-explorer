pub mod command;
pub mod engine;
pub mod interface;
pub mod menu;

use tauri::Manager;

pub struct AppStateRaw {
    pub engine_handle: engine::EngineHandle,
}

pub type AppState = tokio::sync::Mutex<AppStateRaw>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(AppState::new(AppStateRaw {
                engine_handle: engine::spawn_engine(),
            }));
            menu::build_menu(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::parse_source,
            command::reveal_entity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
