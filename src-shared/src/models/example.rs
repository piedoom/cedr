use crate::Id;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Example {
    pub id: Id,
    pub text: String,
    pub translations: String,
    pub source: String, //Source,
}
