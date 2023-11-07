use std::collections::HashMap;

use serde_wasm_bindgen::to_value;
use shared::models;
use wasm_bindgen::JsValue;

use crate::invoke;

pub async fn get(id: u32) -> Result<models::CollectionWithEntries, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke("collections_get", to_value(&HashMap::from([("id", id)]))?).await,
    )
}

pub async fn index() -> Result<Vec<models::Collection>, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(invoke("collections_index", JsValue::null()).await)
}
