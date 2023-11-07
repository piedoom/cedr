use shared::models;
use wasm_bindgen::JsValue;

use crate::invoke;

pub async fn update_cedict() {
    invoke("update_cedict", JsValue::null()).await;
}

pub async fn export_collections() {
    invoke("export_collections", JsValue::null()).await;
}

pub async fn import_collections() {
    invoke("import_collections", JsValue::null()).await;
}

pub async fn get_preferences() -> Result<models::Preferences, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(invoke("get_preferences", JsValue::null()).await)
}

pub async fn set_preferences(preferences: models::Preferences) {
    invoke(
        "set_preferences",
        serde_wasm_bindgen::to_value(&preferences).unwrap(),
    )
    .await;
}
