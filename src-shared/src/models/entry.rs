//! All associated items under a term

use chrono::NaiveDateTime;

use crate::{Id};

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Entry {
    pub id: Id,
    #[cfg(target_arch = "wasm32")]
    pub source_id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub source_id: i64,
    pub traditional: String,
    pub simplified: String,
    /// Pinyin with numbers
    /// Example: `this3 is1 my4 text2`
    pub pinyin_numbers: String,
    pub pinyin_raw: String,
    /// Pinyin with diacritic marks
    pub pinyin: String,
    pub tones: String,
    pub definition: String,
    pub updated_at: NaiveDateTime,
}

impl Entry {
    pub fn tones_u8(&self) -> Vec<u8> {
        self.tones
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(5) as u8)
            .collect()
    }
}
