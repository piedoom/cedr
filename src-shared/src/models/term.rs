use std::hash::Hash;

use itertools::Itertools;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Term {
    #[cfg(target_arch = "wasm32")]
    pub id: u32,
    #[cfg(not(target_arch = "wasm32"))]
    pub id: i64,
    pub traditional: String,
    pub simplified: String,
    /// Pinyin with no tone information
    pub pinyin_raw: String,
    /// Pinyin with numbers
    /// Example: `this3 is1 my4 text2`
    pub pinyin_numbers: String,
    /// Pinyin with diacritic marks
    pub pinyin: String,
    pub tones: String,
}

impl Hash for Term {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.traditional.hash(state);
    }
}

impl Term {
    pub fn tones_u8(&self) -> Vec<u8> {
        self.tones
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(5) as u8)
            .collect()
    }
}

#[derive(Clone, PartialEq)]
pub struct Tones(Vec<u8>);

impl std::ops::Deref for Tones {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Tones {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = self.iter().join("");
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Tones {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ToneVisitor)
    }
}

struct ToneVisitor;

impl<'de> Visitor<'de> for ToneVisitor {
    type Value = Tones;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with numbers representing tones")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut result = Vec::<u8>::default();
        for c in v.chars() {
            let digit = c.to_digit(10).ok_or(E::invalid_value(
                serde::de::Unexpected::Char(c),
                &"must be number",
            ))? as u8;
            result.push(digit);
        }
        Ok(Tones(result))
    }
}
