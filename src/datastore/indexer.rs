use crate::index::error::IndexError;
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
pub enum IndexType {
    Unique,
    Tag,
    Range,
    Inverted,
}

#[derive(Default)]
pub(crate) struct SyncIndexer {
    field_to_index: RwLock<HashMap<String, IndexType>>,
}
impl SyncIndexer {
    pub fn new() -> Self {
        SyncIndexer {
            field_to_index: RwLock::new(HashMap::default()),
        }
    }

    pub fn define_index(
        &self,
        field_name: impl Into<String>,
        index_type: IndexType,
    ) -> Result<(), IndexError> {
        let field_name_string = field_name.into();
        let mut guard = self.field_to_index.write();
        if guard.get(&field_name_string).is_some() {
            return Err(IndexError::Duplicate);
        }
        guard.insert(field_name_string, index_type);
        Ok(())
    }

    pub fn remove_index(&self, field_name: &String, index_type: IndexType) {
        self.field_to_index.write().remove(field_name);
    }

    pub fn find(&self, field_name: &String) -> Option<IndexType> {
        self.field_to_index.read().get(field_name).cloned()
    }
}
