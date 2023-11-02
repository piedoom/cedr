pub mod models;

#[derive(Copy, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum InputMethod {
    Auto = 0,
    Pinyin = 1,
    Chinese = 2,
    English = 3,
}

#[cfg(target_arch = "wasm32")]
pub type Int = u32;
#[cfg(not(target_arch = "wasm32"))]
pub type Int = i64;

impl std::fmt::Display for InputMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
