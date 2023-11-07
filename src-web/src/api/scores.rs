use shared::models;
use wasm_bindgen::JsValue;

use crate::invoke;

pub async fn get(entry_id: u32) -> Result<models::Score, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "scores_get",
            to_value(&HashMap::from([("entry_id", entry_id)]))?,
        )
        .await,
    )
}

pub async fn get_or_create(entry_id: u32) -> Result<models::Score, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "scores_get_or_create",
            to_value(&HashMap::from([("entry_id", entry_id)]))?,
        )
        .await,
    )
}

#[derive(serde::Serialize)]
struct UpdateArgs {
    entry_id: u32,
    status: bool,
}

pub async fn update(
    entry_id: u32,
    correct: bool,
) -> Result<Vec<models::Collection>, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "scores_update",
            serde_wasm_bindgen::to_value(&UpdateArgs { entry_id, correct })?,
        )
        .await,
    )
}
