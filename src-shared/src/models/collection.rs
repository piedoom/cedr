use std::fmt::Display;

use crate::Id;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Collection {
    pub id: Id,
    pub name: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Default, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CollectionWithEntries {
    pub collection: Collection,
    pub entries: Vec<crate::models::Entry>,
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
