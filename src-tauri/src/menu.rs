use tauri::Emitter;
use tauri_plugin_dialog::{DialogExt, FilePath};

use std::io::Read;

use crate::interface::SourceCode;

fn open_file(file_path: FilePath) -> Result<SourceCode, String> {
    let path = file_path
        .into_path()
        .map_err(|e| format!("Selected file path can't be converted into file path. {e}"))?;
    let mut file =
        std::fs::File::open(&path).map_err(|e| format!("Selected file can't be open. {e}"))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("File content can't be read. {e}"))?;
    Ok(SourceCode {
        path: path.into_os_string().into_string().unwrap(),
        content,
    })
}

pub fn build_menu(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file_menu = tauri::menu::SubmenuBuilder::new(app, "File")
        .text("open", "Open")
        .build()?;
    let menu = tauri::menu::MenuBuilder::new(app)
        .items(&[&file_menu])
        .build()?;

    app.set_menu(menu)?;
    app.on_menu_event(
        move |app_handle: &tauri::AppHandle, event| match event.id().0.as_str() {
            "open" => {
                if let Some(file_path) = app_handle.dialog().file().blocking_pick_file() {
                    match open_file(file_path) {
                        Ok(source_code) => app_handle
                            .emit_to(
                                tauri::EventTarget::labeled("main"),
                                "file-picked",
                                source_code,
                            )
                            .unwrap_or_else(|e| log::error!("{e}")),
                        Err(e) => {
                            log::error!("{e}");
                        }
                    }
                }
            }
            _ => {}
        },
    );
    Ok(())
}
