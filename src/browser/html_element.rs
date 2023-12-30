use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Node, HtmlElement};

use super::get_element_by_id;

pub fn get_html_element_by_id(id: &str) -> Result<HtmlElement> {
    get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlElement", element))
}

pub fn append_child(parent: &HtmlElement, child: &Node) -> Result<Node> {
    parent
        .append_child(child)
        .map_err(|_| anyhow!("Error appending child to parent"))
}

pub fn is_scrolled_to_bottom(element: &HtmlElement) -> bool {
    element.scroll_top() + element.client_height() == element.scroll_height()
}
