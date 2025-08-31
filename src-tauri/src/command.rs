use std::{fmt::Debug, io::Write};

use tauri::Manager;

fn stringify<E: Debug>(e: E) -> String {
    format!("{e:?}")
}

#[tauri::command]
pub async fn generate_ast(
    app_handle: tauri::AppHandle,
    source_text: String,
    file_name: String,
) -> Result<String, String> {
    let mut tmp_file_path = app_handle.path().temp_dir().map_err(stringify)?;
    tmp_file_path.push(file_name);
    let mut file = std::fs::File::create(&tmp_file_path).map_err(stringify)?;
    file.write_all(source_text.as_bytes()).map_err(stringify)?;
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, true, true);
    let tu = index.parser(tmp_file_path).parse()?;
    Ok(tu
        .get_entity()
        .get_child(0)
        .unwrap()
        .get_type()
        .map_or(String::from("get_name failed."), stringify))
}
