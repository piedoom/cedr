use shared::models;

use crate::invoke;

#[derive(serde::Serialize)]
struct GetIdArgs {
    traditional: char,
}

pub async fn get_by_traditional(
    traditional: char,
) -> Result<models::Entry, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(
        invoke(
            "entry_get_by_traditional",
            serde_wasm_bindgen::to_value(&GetIdArgs { traditional })?,
        )
        .await,
    )
}
