use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AstEntityLite {
    pub id: String,
    pub kind: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstEntityFull {
    pub properties: Vec<(String, String)>,
    pub children: Vec<AstEntityLite>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceCode {
    pub path: String,
    pub content: String,
}
