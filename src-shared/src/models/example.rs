use super::Source;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Example {
    #[cfg(target_arch = "wasm32")]
    pub id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub id: i64,
    pub text: String,
    pub translations: String,
    pub source: String, //Source,
}
