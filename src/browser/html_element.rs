use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::get_element_by_id;

pub fn get_html_element_by_id(id: &str) -> Result<HtmlElement> {
    get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlElement", element))
}
