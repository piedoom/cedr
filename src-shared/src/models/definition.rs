#[derive(Default, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Definition {
    #[cfg(target_arch = "wasm32")]
    pub id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub id: i64,
    pub definition: String,
    #[cfg(target_arch = "wasm32")]
    pub source_id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub source_id: i64,
    pub term: String,
    #[cfg(target_arch = "wasm32")]
    pub hash: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub hash: i64,
    pub updated_at: chrono::NaiveDateTime,
}
