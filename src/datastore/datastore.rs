use bson::{oid::ObjectId, Document, Bson};
use dashmap::{iter::Iter, DashMap, mapref::one::Ref};

use crate::index::{
    hash::HashIndex, inverted_index::InvertedIndex, range::RangeIndex, tags::TagIndex,
};

use super::{
    config::Config,
    error::Error,
    indexer::{IndexType, Indexer, SyncIndexer, ViewFilter},
};

pub const ObjectID: &'static str = "_id";

#[derive(Default)]
pub struct DataStore {
    data_store: DashMap<ObjectId, Document>,

    // HashIndex
    hash_index: HashIndex,

    // TagIndex
    tag_index: TagIndex,

    // RangeIndex
    range_index: RangeIndex,

    // InvertedIndex
    inverted_index: InvertedIndex,

    // Indexer
    indexer: SyncIndexer,

    cfg: Config,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore::default()
    }

    pub fn with_indexer(mut self, idx: Indexer) -> Self {
        self.indexer = SyncIndexer::new(idx);
        return self;
    }

    pub fn define_index(&self, field_name: impl Into<String>, index_type: IndexType) {
        let mut guard = self.indexer.write();
        guard.define_index(field_name, index_type);
    }

    pub fn remove_index(&self, field_name: &String, index_type: IndexType) {
        let mut guard = self.indexer.write();
        guard.remove_index(field_name, index_type);
    }


    pub fn insert(&self, mut doc: Document) -> Result<Option<Document>, Error> {
        let oid;
        match doc.get(ObjectID) {
            Some(object_id) => {
                match object_id.as_object_id() {
                    Some(object_id) => oid = object_id,
                    None => return Err(Error::InvalidObjectId),
                };
            }
            None => {
                let temp = ObjectId::new();
                doc.insert(ObjectID, temp.clone());
                oid = temp;
            }
        };

        let guard = self.indexer.write();

        for index in 0..guard.hash_fields.len() {
            let field_name = &guard.hash_fields[index];
            if let Some(reference) = doc.get(field_name) {
                if let Err(e) = self.hash_index.insert(reference.to_string(), oid.clone()) {
                    // RollBack HashIndex
                    let mut from = index;
                    loop {
                        let field_name = &guard.hash_fields[from];
                        if let Some(reference) = doc.get(field_name) {
                            self.hash_index.remove(&reference.to_string());
                        }

                        if from == 0 {
                            return Err(Error::IndexError(e));
                        }

                        from -= 1;
                    }
                }
            }
        }

        guard
            .tag_fields
            .iter()
            .for_each(|tag| self.tag_index.insert(tag.to_owned(), oid.clone()));

        guard.range_fields.iter().for_each(|field_name| {
            doc.get(field_name).map(|value| {
                self.range_index
                    .insert(field_name.to_owned(), value.to_string(), oid.clone());
            });
        });

        guard.inverted_index_fields.iter().for_each(|field_name| {
            doc.get(field_name).map(|value| {
                value.as_str().map(|s| {
                    self.inverted_index.insert(&oid, s);
                });
            });
        });

        guard.view_filters.iter().for_each(|(view_name, filter)| {
            filter(&doc).then(|| {
                self.tag_index.insert_view(view_name, oid.clone());
            });
        });

        Ok(self.data_store.insert(oid, doc))
    }

    pub fn remove(&self, object_id: &ObjectId) {
        self.data_store.remove(object_id).map(|(object_id, doc)| {
            let guard = self.indexer.write();

            guard.hash_fields.iter().for_each(|field_name| {
                doc.get(field_name).map(|reference| {
                    self.hash_index.remove(&reference.to_string());
                });
            });

            guard
                .tag_fields
                .iter()
                .for_each(|tag| self.tag_index.remove(tag, &object_id));

            guard.range_fields.iter().for_each(|field_name| {
                if let Some(value) = doc.get(field_name) {
                    self.range_index
                        .remove(field_name, &value.to_string(), &object_id);
                }
            });

            guard.inverted_index_fields.iter().for_each(|field_name| {
                if let Some(value) = doc.get(field_name) {
                    if let Some(s) = value.as_str() {
                        self.inverted_index.remove(&object_id, s);
                    }
                }
            });
        });
    }


    pub fn find(&self, object_id: &ObjectId) -> Option<Ref<ObjectId, Document>>{
        self.data_store.get(object_id)
    }

    pub fn lookup(&self, field_name: &String, field_value: &Bson) {
        match self.indexer.find(field_name) {
            Some(idx_type) => {
                self.
            }
            None => {
                
            }
        }
    }



    pub fn iter(&self) -> Iter<ObjectId, Document> {
        self.data_store.iter()
    }
}
