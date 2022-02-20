pub mod console;
pub mod reply_tree;

use crate::DOM_PARSE_ERR_MSG;
use wasm_bindgen::prelude::*;

const WAIT_LIMIT_SEC: u16 = 10;
const SLEEP_MS: u16 = 500;

/// Wait dom ready.
#[wasm_bindgen]
pub async fn wait_dom_ready() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or(DOM_PARSE_ERR_MSG)?
        .document()
        .ok_or(DOM_PARSE_ERR_MSG)?;
    let mut counter = 0;

    while document.query_selector("section")?.is_none() {
        if counter * SLEEP_MS > WAIT_LIMIT_SEC * 1000 {
            return Err("Failed to load Reply Tweets. Time exeeded.".into());
        }
        crate::utils::sleep_ms(SLEEP_MS).await?;
        counter += 1;
    }

    Ok(())
}
