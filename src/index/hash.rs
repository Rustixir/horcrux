use bson::oid::ObjectId;
use dashmap::{DashMap, mapref::one::Ref, iter::Iter};
use super::error::IndexError;


#[derive(Default)]
pub struct HashIndex {
    hash: DashMap<String, ObjectId>,
}

impl HashIndex {

    pub fn new() -> Self {
        HashIndex {
            hash: DashMap::new(),
        }
    }

    /// insert entry
    #[inline]
    pub fn insert(&self, reference: String, object_id: ObjectId) -> Result<(), IndexError>{
        if self.hash.get(&reference).is_some() {
            return Err(IndexError::Duplicate)
        }
        self.hash.insert(reference, object_id);
        Ok(())
    }

    /// remove entry
    #[inline]
    pub fn remove(&self, reference: &String) {
       self.hash.remove(reference);
    }

    /// lookup by index_key
    #[inline]
    pub fn lookup(&self, reference: &String) -> Option<Ref<String, ObjectId>> {
        self.hash.get(reference)
    }

    /// get iter
    #[inline]
    pub fn iter(&self) -> Iter<String, ObjectId> {
        self.hash.iter()
    }

}
