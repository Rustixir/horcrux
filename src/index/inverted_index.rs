use bson::oid::ObjectId;
use dashmap::{DashMap, DashSet};
use std::{sync::Arc, collections::HashSet};




#[derive(Default)]
pub struct InvertedIndex {
    index: Arc<DashMap<String, DashSet<ObjectId>>>,
}

impl InvertedIndex {

    pub fn new() -> Self {
        InvertedIndex { 
            index: Arc::new(DashMap::new()),
        }
    }


    /// insert entry
    #[inline]
    pub fn insert(&self, object_id: &ObjectId, content: &str) {
        for word in content.split_whitespace() {
            let word = word.to_lowercase() ;
            match self.index.get_mut(&word) {
                Some(list) => {
                    if list.value().get(object_id).is_none() {
                        list.value().insert(object_id.to_owned());
                    }
                }
                None => {
                    let list = DashSet::new();
                    list.insert(object_id.to_owned());
                    self.index.insert(word, list);
                }
            }
        }
    }

    /// remove entry
    #[inline]
    pub fn remove(&self, object_id: &ObjectId, content: &str) {
        for word in content.split_whitespace() {
            let word = word.to_lowercase() ;
            if let Some(list) = self.index.get_mut(&word) {
                list.value().remove(object_id);
            }
        }
    }
   
   
    /// search by text
    #[inline]
    pub fn search(&self, text: &String) -> Vec<ObjectId> {
        let words = text.split_whitespace();
        let mut collector = HashSet::new();
        for w in words {
            let object_ids = self.inner_search(w);
            self.intersect(object_ids, &mut collector);
        }

        collector.into_iter().collect()
    }

    #[inline] 
    fn intersect(&self, object_ids: Vec<ObjectId>, collector: &mut HashSet<ObjectId>) {
        for obj_id in object_ids {
            if collector.get(&obj_id).is_none() {
                collector.insert(obj_id);
            }
        }
    }


    #[inline] 
    fn inner_search(&self, word: &str) -> Vec<ObjectId> {
        let word = word.to_lowercase();
        match self.index.get(&word) {
            Some(list) => {
                list
                    .value()
                    .iter()
                    .map(|entry| entry.to_owned() )
                    .collect()
            }
            None => {
                vec![]
            }        
        }
    }



}

