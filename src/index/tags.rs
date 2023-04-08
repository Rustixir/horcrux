use bson::oid::ObjectId;
use dashmap::{iter::Iter, mapref::one::Ref, DashMap, DashSet};


#[derive(Default)]
pub struct TagIndex {
    pub tags: DashMap<String, DashSet<ObjectId>>,
}

impl TagIndex {
    pub fn new() -> Self {
        TagIndex {
            tags: DashMap::new(),
        }
    }

    /// insert entry with tags
    #[inline]
    pub fn insert(&self, tag: String, object_id: ObjectId) {
        match self.tags.get_mut(&tag) {
            Some(set) => {
                set.value().insert(object_id);
            }
            None => {
                let set = DashSet::new();
                set.insert(object_id);
                self.tags.insert(tag, set);
            }
        }
    }

    /// insert entry with tags
    #[inline]
    pub fn insert_view(&self, view_name: &str, object_id: ObjectId) {
        let view_key = self.view_key_maker(view_name);

        match self.tags.get_mut(&view_key) {
            Some(set) => {
                set.value().insert(object_id);
            }
            None => {
                let set = DashSet::new();
                set.insert(object_id);
                self.tags.insert(view_key, set);
            }
        }
    }

    /// remove entry from tags
    #[inline]
    pub fn remove(&self, tag: &String, object_id: &ObjectId) {
        if let Some(set) = self.tags.get_mut(tag) {
            set.value().remove(&object_id);
        }
    }

    /// remove entry from view
    #[inline]
    pub fn remove_from_view(&self, view_name: &str, object_id: &ObjectId) {
        let view_key = &self.view_key_maker(view_name);
        if let Some(set) = self.tags.get_mut(view_key) {
            set.value().remove(&object_id);
        }
    }

    /// remove tag
    #[inline]
    pub fn remove_tag(&self, tag: &str) {
        self.tags.remove(tag);
    }

    /// remove view
    #[inline]
    pub fn remove_view(&self, view_name: &str) {
        let view_key = self.view_key_maker(view_name);
        self.tags.remove(&view_key);
    }

    /// lookup by tag
    #[inline]
    pub fn lookup(&self, tag: &str) -> Option<Ref<String, DashSet<ObjectId>>> {
        self.tags.get(tag)
    }

    /// lookup by tag
    #[inline]
    pub fn lookup_view(&self, view_name: &str) -> Option<Ref<String, DashSet<ObjectId>>> {
        self.tags.get(&self.view_key_maker(view_name))
    }

    /// get iter
    #[inline]
    pub fn iter(&self) -> Iter<String, DashSet<ObjectId>> {
        self.tags.iter()
    }

    #[inline]
    fn view_key_maker(&self, name: &str) -> String {
        format!("__View__{}", name)
    }
}
