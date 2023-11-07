use std::collections::HashMap;

use serde_wasm_bindgen::to_value;
use shared::models;

use crate::invoke;

pub async fn get_by_traditional(
    traditional: char,
) -> Result<models::Entry, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "entry_get_by_traditional",
            to_value(&HashMap::from([("traditional", traditional)]))?,
        )
        .await,
    )
}
