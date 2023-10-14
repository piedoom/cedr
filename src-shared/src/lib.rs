pub mod models;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum InputMethod {
    Auto = 0,
    Pinyin = 1,
    Chinese = 2,
    English = 3,
}

impl std::fmt::Display for InputMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
