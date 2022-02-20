//! # `GazoRepKiller`
//! Remove Gazo reply from Tweet reply tree.

// Supress `#[wasm_bindgen]` unneeded unit expression error.
#![allow(clippy::unused_unit)]

mod dom_util;
mod utils;
mod walker;
use dom_util::reply_tree::ReplyTree;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
use std::panic;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
const DOM_PARSE_ERR_MSG: &str = "DOM Parsing Error";

/// If Reply tree has renewed, search tweet and kill gazorep.
/// Else, wait a sec and try again.
#[wasm_bindgen]
pub async fn killing() -> Result<(), JsValue> {
    #[allow(non_snake_case)]
    let REPLY_TREE_ROOT: web_sys::Element = <web_sys::Element as ReplyTree>::create_root()?;
    // only use as tweet identifier
    // ReplyTree last element may be empty.
    // Empty element's text_content() equals to "" so String::new() can't detect renew or not.
    let mut last_reply_text = String::from("_");
    let mut old_url = utils::get_current_url()?;

    loop {
        // if url not same, exit function to reload.
        let url = utils::get_current_url()?;
        if url != old_url {
            return Err(JsValue::from_str("URL not match. Reloading..."));
        }
        old_url = url;

        // check reply tree has renewed.
        if last_reply_text == REPLY_TREE_ROOT.get_last_text()? {
            // sleep 500ms
            utils::sleep_ms(500).await?;
        } else {
            walker::search_and_kill(&REPLY_TREE_ROOT)?;
            last_reply_text = REPLY_TREE_ROOT.get_last_text()?;
        }
    }
}

/// Entrypoint
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    loop {
        // First, check url match
        // cuz twitter.com not match the manifest.json content_scripts pattern.
        // So, made content_script url pattern `twitter.com`.
        let url = utils::get_current_url()?;
        if !utils::check_url_match(&url) {
            utils::sleep_ms(5000).await?;
            continue;
        }

        // Then, url matched, wait for DOM ready.
        // If DOM not ready, sleep 500ms and try again.
        // If DOM ready, start killing.
        match dom_util::wait_dom_ready().await {
            Ok(_) => {
                if let Err(e) = killing().await {
                    console_log!("Oops! Error occurred while killing: {:?}", e);
                    utils::sleep_ms(500).await?;
                    continue;
                }
            }
            Err(_e) => {
                utils::sleep_ms(500).await?;
                continue;
            }
        }
    }
}
