use crate::Int;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Example {
    pub id: Int,
    pub text: String,
    pub translations: String,
    pub source: String, //Source,
}
