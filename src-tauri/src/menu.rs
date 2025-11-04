use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;

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
                    if let Err(e) = app_handle.emit_to(
                        tauri::EventTarget::labeled("main"),
                        "file-picked",
                        match &file_path {
                            tauri_plugin_dialog::FilePath::Url(url) => url.to_string(),
                            tauri_plugin_dialog::FilePath::Path(path) => {
                                format!("file:///{}", path.to_string_lossy())
                            }
                        },
                    ) {
                        log::error!(
                            "File ({:?}) has been selected, but failed to convey to UI. {e}",
                            file_path.as_path()
                        );
                    }
                }
            }
            _ => {}
        },
    );
    Ok(())
}
