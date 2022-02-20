use crate::DOM_PARSE_ERR_MSG;
use wasm_bindgen::prelude::*;

/// Search tweet text by dom tree index.
/// This method may incompatible with future varsion of twitter.
#[wasm_bindgen]
pub fn search_text_element(article_element: web_sys::Element) -> Result<web_sys::Element, JsValue> {
    let mut reply_search_element = article_element;
    while reply_search_element.children().length() == 1 {
        reply_search_element = reply_search_element
            .first_element_child()
            .ok_or(DOM_PARSE_ERR_MSG)?;
    }
    while reply_search_element.children().length() > 1 {
        // reply's reply dom tree.
        // This is not same as replyElem's reply dom tree.
        if reply_search_element.children().length() == 3 {
            reply_search_element = reply_search_element
                .first_element_child()
                .ok_or(DOM_PARSE_ERR_MSG)?;
            continue;
        }
        reply_search_element = reply_search_element
            .children()
            .get_with_index(1)
            .ok_or(DOM_PARSE_ERR_MSG)?;
    }

    Ok(reply_search_element)
}

/// Search tweet article by dom tree index.
/// This method may incompatible with future varsion of twitter.
#[wasm_bindgen]
pub fn search_article(reply_element: &web_sys::Element) -> Result<web_sys::Element, JsValue> {
    let reply_search_element = reply_element
        .query_selector("article")?
        .ok_or(DOM_PARSE_ERR_MSG)?;

    Ok(reply_search_element)
}
