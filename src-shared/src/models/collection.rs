use crate::Id;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Collection {
    pub id: Id,
    pub name: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionWithEntries {
    pub collection: Collection,
    pub entries: Vec<crate::models::Entry>,
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub mod export {
    use serde::{Deserialize, Serialize};

    pub type Collections = Vec<Collection>;

    #[derive(Serialize, Deserialize)]
    pub struct Collection {
        /// The name of the collection
        pub name: String,
        /// Traditional characters resolving to an entry
        pub entries: Vec<String>,
    }
}
