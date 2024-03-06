use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Element, Node, HtmlElement};

use super::document;

pub fn get_element_by_id(id: &str) -> Result<Element> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("No Element found with ID {}", id))
}

pub fn create_element(tag: &str) -> Result<Element> {
    document()?
        .create_element(tag)
        .map_err(|_| anyhow!("Error creating element with tag {}", tag))
}

pub fn crate_element_from_node(node: Node) -> Result<Element> {
    node
        .dyn_into::<Element>()
        .map_err(|node| anyhow!("Error converting {:#?} to Element", node))
}

pub fn create_element_from_html_element(html_element: HtmlElement) -> Result<Element> {
    html_element
        .dyn_into::<Element>()
        .map_err(|element| anyhow!("Error converting {:#?} to Element", element))
}

pub fn element_text_content(element: &Element) -> Result<String> {
    element.text_content()
        .ok_or_else(|| anyhow!("No text content found for element"))
}

pub fn has_class_element(element: &Element, class: &str) -> bool {
    element.class_list().contains(class)
}

pub fn element_append_child(parent: &Element, child: &Node) -> Result<Node> {
    parent
        .append_child(child)
        .map_err(|_| anyhow!("Error appending child to parent"))
}

pub fn element_query_selector(element: &Element, selector: &str) -> Result<Element> {
    element
        .query_selector(selector)
        .map_err(|_| anyhow!("No element found for selector {}", selector))?
        .ok_or_else(|| anyhow!("No element found for selector {}", selector))
}

// pub fn get_element_from_event(event: &Event) -> Result<Element> {
//     event
//         .target()
//         .ok_or_else(|| anyhow!("No target found on event"))?
//         .dyn_into::<Element>()
//         .map_err(|element| anyhow!("Error converting {:#?} to Element", element))
// }
