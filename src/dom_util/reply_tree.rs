use crate::DOM_PARSE_ERR_MSG;
use wasm_bindgen::prelude::*;

/// function about the reply tree root.
/// These method chain may incompatible with future varsion of twitter.
pub trait ReplyTree {
    fn get_last_text(&self) -> Result<String, JsValue>;
    fn create_root() -> Result<web_sys::Element, JsValue>;
}

impl ReplyTree for web_sys::Element {
    /// Get the last tweet text.
    /// The text useing as tweet identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut last_reply_text = REPLY_TREE_ROOT.get_last_text()?;
    /// loop {
    ///     if last_reply_text != REPLY_TREE_ROOT.get_last_text()? {
    ///         // Reply Tree has been updated.
    ///         processing(&REPLY_TREE_ROOT)?;
    ///         last_reply_text = REPLY_TREE_ROOT.get_last_text()?;
    ///     } else {
    ///         // not updated. sleep 500ms
    ///         sleep!(500);
    ///     }
    /// }
    /// ```
    fn get_last_text(&self) -> Result<String, JsValue> {
        let text = self
            .last_element_child()
            .ok_or(DOM_PARSE_ERR_MSG)?
            .text_content()
            .ok_or(DOM_PARSE_ERR_MSG)?;
        Ok(text)
    }

    /// Create a `ReplyTree` root element from window.
    fn create_root() -> Result<web_sys::Element, JsValue> {
        let reply_tree_root: web_sys::Element = web_sys::window()
            .ok_or(DOM_PARSE_ERR_MSG)?
            .document()
            .ok_or(DOM_PARSE_ERR_MSG)?
            .body()
            .ok_or(DOM_PARSE_ERR_MSG)?
            .query_selector("section")?
            .ok_or(DOM_PARSE_ERR_MSG)?
            .children()
            .get_with_index(1)
            .ok_or(DOM_PARSE_ERR_MSG)?
            .first_element_child()
            .ok_or(DOM_PARSE_ERR_MSG)?;

        Ok(reply_tree_root)
    }
}
