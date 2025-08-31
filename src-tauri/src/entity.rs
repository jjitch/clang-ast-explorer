use std::collections::HashMap;

pub trait AstEntity: Sized {
    fn children(&self) -> Vec<Self>;
    fn properties(&self) -> HashMap<String, String>;
}

struct ClangEntity<'a> {
    entity: clang::Entity<'a>,
}

impl<'a> From<clang::Entity<'a>> for ClangEntity<'a> {
    fn from(value: clang::Entity<'a>) -> Self {
        Self { entity: value }
    }
}

impl<'a> AstEntity for ClangEntity<'a> {
    fn children(&self) -> Vec<Self> {
        self.entity
            .get_children()
            .into_iter()
            .map(|e| ClangEntity { entity: e })
            .collect()
    }
    fn properties(&self) -> HashMap<String, String> {
        HashMap::<String, String>::new()
    }
}

pub struct EntityLinker<E: AstEntity> {
    entities_pool: Vec<E>,
}

impl<E: AstEntity> EntityLinker<E> {
    pub fn new() -> Self {
        Self {
            entities_pool: Vec::<E>::new(),
        }
    }
    pub fn populate_id(&mut self, e: E) -> usize {
        let id = self.entities_pool.len();
        self.entities_pool.push(e);
        id
    }

    pub fn resolve_entity(&self, id: usize) -> &E {
        &self.entities_pool[id]
    }
}
