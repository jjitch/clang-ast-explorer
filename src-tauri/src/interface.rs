use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AstEntityLite {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstEntityFull {
    pub properties: Vec<(String, String)>,
    pub children: Vec<AstEntityLite>,
}
