//! All associated items under a term

use crate::models;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Entry {
    pub term: models::Term,
    pub definitions: Vec<models::Definition>,
    pub examples: Vec<models::Example>,
}
