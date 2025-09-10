use clang_ast_explorer_lib::engine::*;

#[tokio::test]
async fn test_compile_source_code() {
    let engine = spawn_engine();
    let source_code = r#"
#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::end
    return 0;
}
"#;
    let temp_dir = tempdir::TempDir::new("ast-explorer-test_compile_source_code").unwrap();
    let source_file = temp_dir.path().join("test.cpp");
    std::fs::write(&source_file, source_code).unwrap();
    let res = engine
        .call(|tx| Msg::CompileSourceCode(tx, source_file))
        .await;
    assert!(res.is_ok());
    let diagnostics = res.unwrap();
    assert!(!diagnostics.is_empty());
    for diag in diagnostics {
        eprintln!(
            "{}:{}: {}: {}",
            diag.file, diag.line, diag.severity, diag.message
        );
    }
}
