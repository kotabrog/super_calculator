use anyhow::Result;
use crate::browser::{
    query_selector_all, node_list_get, 
    node_list_length
};
use super::Node;

#[derive(Debug, Clone)]
pub struct NodeList {
    inner: web_sys::NodeList,
}

impl NodeList {
    pub fn new(inner: web_sys::NodeList) -> Self {
        Self { inner }
    }

    pub fn new_from_selector(selector: &str) -> Result<Self> {
        Ok(Self::new(query_selector_all(selector)?))
    }

    pub fn length(&self) -> u32 {
        node_list_length(&self.inner)
    }

    pub fn get(&self, index: u32) -> Result<Node> {
        Ok(Node::new(node_list_get(&self.inner, index)?))
    }
}
