use anyhow::{anyhow, Result};
use web_sys::{Element, Node};

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

pub fn element_append_child(parent: &Element, child: &Node) -> Result<Node> {
    parent
        .append_child(child)
        .map_err(|_| anyhow!("Error appending child to parent"))
}
