use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Node, HtmlElement};

use super::get_element_by_id;

pub fn get_html_element_by_id(id: &str) -> Result<HtmlElement> {
    get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlElement", element))
}

pub fn get_html_element_from_event(event: &web_sys::Event) -> Result<HtmlElement> {
    event
        .target()
        .ok_or_else(|| anyhow!("No target found on event"))?
        .dyn_into::<HtmlElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlElement", element))
}

pub fn has_class(element: &HtmlElement, class: &str) -> bool {
    element.class_list().contains(class)
}

pub fn set_class(element: &HtmlElement, class: &str) {
    element.set_class_name(class)
}

pub fn add_class(element: &HtmlElement, class: &str) -> Result<()> {
    element.class_list()
        .add_1(class)
        .map_err(|_| anyhow!("Error adding class {}", class))
}

pub fn remove_class(element: &HtmlElement, class: &str) -> Result<()> {
    element.class_list()
        .remove_1(class)
        .map_err(|_| anyhow!("Error removing class {}", class))
}

pub fn append_child(parent: &HtmlElement, child: &Node) -> Result<Node> {
    parent
        .append_child(child)
        .map_err(|_| anyhow!("Error appending child to parent"))
}

pub fn is_scrolled_to_bottom(element: &HtmlElement) -> bool {
    element.scroll_top() + element.client_height() == element.scroll_height()
}
