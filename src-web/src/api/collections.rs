use shared::models;
use wasm_bindgen::JsValue;

use crate::invoke;

#[derive(serde::Serialize)]
pub struct CollectionArgs {
    id: u32,
}

pub async fn get(id: u32) -> Result<models::CollectionWithEntries, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "collections_get",
            serde_wasm_bindgen::to_value(&CollectionArgs { id })?,
        )
        .await,
    )
}

pub async fn index() -> Result<Vec<models::Collection>, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(invoke("collections_index", JsValue::null()).await)
}
