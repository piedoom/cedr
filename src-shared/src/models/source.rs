use std::str::FromStr;

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Source {
    #[cfg(target_arch = "wasm32")]
    pub id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub id: i64,
    pub name: String,
    pub kind: SourceKind,
    pub url: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Debug)]
pub enum SourceKind {
    Terms,
    Examples,
}

impl FromStr for SourceKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "terms" => Ok(Self::Terms),
            "examples" => Ok(Self::Examples),
            _ => Err("incorrect source kind type".to_string()),
        }
    }
}

impl ToString for SourceKind {
    fn to_string(&self) -> String {
        match self {
            SourceKind::Terms => "terms",
            SourceKind::Examples => "examples",
        }
        .to_string()
    }
}
