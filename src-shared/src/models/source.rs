use crate::Int;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Source {
    pub id: Int,
    pub name: String,
    pub url: String,
    pub license: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}
