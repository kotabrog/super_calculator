use anyhow::{anyhow, Result};
use web_sys::{NodeList, Node};

pub fn node_list_length(node_list: &NodeList) -> u32 {
    node_list.length()
}

pub fn node_list_get(node_list: &NodeList, index: u32) -> Result<Node> {
    node_list
        .get(index)
        .ok_or_else(|| anyhow!("No node found at index {}", index))
}
