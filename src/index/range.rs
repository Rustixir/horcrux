use std::{collections::{BTreeMap, BTreeSet}, ops::Bound};
use bson::oid::ObjectId;
use dashmap::{DashMap, DashSet};



#[derive(Default)]
pub struct RangeIndex {
    multi_btree: DashMap<String, BTreeMap<String, DashSet<ObjectId>>>,
}

impl RangeIndex {
    
    pub fn new() -> Self {
        RangeIndex {
            multi_btree: DashMap::new(),
        }
    }

    /// insert entry to tree
    #[inline]
    pub fn insert(&self, field_name: String, value: String, object_id: ObjectId) {

        match self.multi_btree.get_mut(&field_name) {
            Some(mut tree) => match tree.get_mut(&value) {
                Some(set) => {
                    set.insert(object_id);
                }
                None => {
                    let set = DashSet::new();
                    set.insert(object_id);
                    tree.value_mut().insert(value, set);
                }
            },
            None => {
                let mut tree = BTreeMap::new();
                let set = DashSet::new();
                set.insert(object_id);
                tree.insert(value, set);
                self.multi_btree.insert(field_name, tree);
            }
        }
    }

    /// remove entry from tree
    #[inline]
    pub fn remove(&self, field_name: &String, value: &String, object_id: &ObjectId) {
        if let Some(mut tree) = self.multi_btree.get_mut(field_name) {
            if let Some(set) = tree.value_mut().get_mut(value) {
                set.remove(object_id);
            }
        }
    }

    /// remove tree from multi-tree
    #[inline]
    pub fn remove_tree(&self, field_name: &str) {
        self.multi_btree.remove(field_name);
    }


    /// fetch document by range hash_index
    #[inline]
    pub fn range(&self, field_name: &str, from: String, to: String) -> Vec<ObjectId> {
        let set = match self.multi_btree.get(field_name) {
            Some(tree) => {
                let mut set_result = BTreeSet::new();

                // collect and distinct references
                for (_, set) in tree.range((Bound::Included(from), Bound::Excluded(to))) {
                    for k in set.iter() {
                        set_result.insert(k.clone());
                    }
                }

                set_result
            }
            None => BTreeSet::new()
        };

        set
        .into_iter()
        .collect::<Vec<ObjectId>>()
    }

    
}
