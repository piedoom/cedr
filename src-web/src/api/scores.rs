use shared::models;
use wasm_bindgen::JsValue;

use crate::invoke;

#[derive(serde::Serialize)]
struct GetArgs {
    entry_id: u32,
}

pub async fn get(entry_id: u32) -> Result<models::Score, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "scores_get",
            serde_wasm_bindgen::to_value(&GetArgs { entry_id })?,
        )
        .await,
    )
}

pub async fn get_or_create(entry_id: u32) -> Result<models::Score, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "scores_get_or_create",
            serde_wasm_bindgen::to_value(&GetArgs { entry_id })?,
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
