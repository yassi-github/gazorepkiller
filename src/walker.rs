use crate::DOM_PARSE_ERR_MSG;
use wasm_bindgen::prelude::*;

pub mod killer;
pub mod searcher;

type TweetArticle = web_sys::Element;
trait TweetArticleSearcher {
    fn search_text_element(self) -> Result<TweetArticle, JsValue>;
}
impl TweetArticleSearcher for TweetArticle {
    fn search_text_element(self) -> Result<TweetArticle, JsValue> {
        searcher::search_text_element(self)
    }
}

/// Search non-text-body tweet and kill it.
/// Dig into each reply tweet that belongs as a child of the ReplyTree.
/// Each reply tweet element may not a tweet.
/// So if that a case, skip element.
#[wasm_bindgen]
pub fn search_and_kill(reply_tree_root: &web_sys::Element) -> Result<(), JsValue> {
    #[allow(non_snake_case)]
    // This veriable is constant but reply_tree_root is not so use `let` instead of `const`.
    let REPLY_TREE: web_sys::HtmlCollection = reply_tree_root.children();

    // Dig into the each reply tree elements.
    // first index is primary tweet, so skip it.
    for reply_idx in 1..REPLY_TREE.length() {
        let reply_element: web_sys::Element = REPLY_TREE
            .get_with_index(reply_idx)
            .ok_or(DOM_PARSE_ERR_MSG)?;
        // skip if the reply_element is not a tweet
        // A zero height element always seems to above the actual tweet.
        if reply_element.client_height() == 0 {
            continue;
        }

        // skip if the reply_element is not a tweet
        // Tweet has a article element.
        let reply_search_element = searcher::search_article(&reply_element);
        let mut reply_search_element: TweetArticle = match reply_search_element {
            Ok(reply_search_element) => reply_search_element,
            Err(_) => {
                // // wait till dom load completed.
                // // sleep 500ms
                // std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }
        };

        reply_search_element = reply_search_element.search_text_element()?;

        // kill tweet
        // if element has no text div element.
        if reply_search_element.children().length() == 0 {
            killer::kill(reply_element)?;
        }
    }

    Ok(())
}
