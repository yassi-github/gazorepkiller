use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Kill tweet.
/// For given argument element, set style to display: none.
#[wasm_bindgen]
pub fn kill(element: web_sys::Element) -> Result<(), JsValue> {
    // web_sys::Element not implemented style method.
    // So to modify the CSS style of the element,
    // we need to cast the element as a HTMLElement.
    let element = element.dyn_into::<web_sys::HtmlElement>()?;
    element.style().set_property("display", "none")?;
    // debug
    // element.style().set_property("border", "5px dashed red")?;
    // crate::gazorepkiller_console_log!("killed {}", element.inner_html());

    Ok(())
}
