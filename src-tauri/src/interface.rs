use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AstEntityLite {
    pub id: String,
    pub kind: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceRange {
    pub start_line: i64,
    pub start_column: i64,
    pub end_line: i64,
    pub end_column: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstEntityFull {
    pub properties: Vec<(String, String)>,
    pub children: Vec<AstEntityLite>,
    pub source_range: Option<SourceRange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceCode {
    pub path: String,
    pub content: String,
}
