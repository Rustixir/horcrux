use std::time::Instant;

pub use bson::{bson, doc, document, oid::ObjectId, Bson, Document};
pub use dashmap::iter::Iter;
use datastore::datastore::DataStore;
pub use serde::{Deserialize, Serialize};

use crate::datastore::indexer::{IndexType, Indexer};
pub mod datastore;
pub mod index;

fn main() {
    let idx = Indexer::new()
        .define_index("_id", IndexType::Unique)
        .define_index("fullname", IndexType::Tag);

    let ds = DataStore::new().with_indexer(idx);

    let now = Instant::now();
    // ---------------------------------

    for counter in 0..400_000 {
        let _ = ds.insert(doc! {
            "_id": ObjectId::new(),
            "age": counter % 18 + 10,
            "fullname": "Danyalmh"
        });
    }

    // ---------------------------------
    let nnow = Instant::now();
    println!("==> {:?}", nnow.duration_since(now))
}
