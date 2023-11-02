use crate::Int;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Score {
    pub id: Int,
    pub level: Int,
    pub entry_id: Int,
    pub active: bool,
    pub updated_at: chrono::NaiveDateTime,
}
