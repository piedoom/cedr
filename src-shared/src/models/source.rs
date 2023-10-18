use crate::Id;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Source {
    pub id: Id,
    pub name: String,
    pub url: String,
    pub license: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}
