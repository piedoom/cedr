use std::fmt::Display;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Collection {
    #[cfg(target_arch = "wasm32")]
    pub id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub id: i64,
    pub name: String,
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
