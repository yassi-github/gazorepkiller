use crate::DOM_PARSE_ERR_MSG;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// Sleep.
/// # Arguments
/// - (u64) - milliseconds
/// # Example
/// ```
/// // sleep 500 ms
/// sleep!(500);
/// ```
// #[macro_export]
// macro_rules! sleep {
//     ($e:expr) => {
//         // can't use thread sleep in wasm.
//         std::thread::sleep(std::time::Duration::from_millis($e));
//         // needs async but macro can't ..?
//         // await crate::utils::sleep_millis($e);
//     };
// }

/// Match url pattern: "*://twitter.com/*/status/*"
/// If not match, return false
#[wasm_bindgen]
pub fn check_url_match(url: &str) -> bool {
    let re = regex::Regex::new(r"^(https?://)?twitter.com/[^/]+/status/[^/]+$");
    match re {
        Ok(re) => re.is_match(url),
        Err(_) => false,
    }
}

/// Get current page url.
#[wasm_bindgen]
pub fn get_current_url() -> Result<String, JsValue> {
    web_sys::window()
        .ok_or(DOM_PARSE_ERR_MSG)?
        .location()
        .href()
}

/// Extern sleep function from JavaScript.
/// In WASM, sleep is so confusing.
/// So, use JavaScript inline function instead.
#[wasm_bindgen(
    inline_js = "export function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }"
)]
extern "C" {
    fn sleep(ms: u16) -> js_sys::Promise;
}

/// Sleep for milliseconds.
/// Using JavaScript Promise, so it and it's calling function is async.
#[wasm_bindgen]
pub async fn sleep_ms(ms: u16) -> Result<(), JsValue> {
    // Can't use thread sleep in wasm.
    // So I use JS inline script :poop:
    JsFuture::from(sleep(ms)).await?;

    Ok(())
}
