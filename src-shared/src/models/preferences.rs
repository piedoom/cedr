use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Preferences {
    pub theme: Theme,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Theme {
    // TODO: Set to default when has ability to do auto
    Auto,
    Light,
    #[default]
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Auto => "auto",
                Self::Light => "light",
                Self::Dark => "dark",
            }
        )
    }
}
