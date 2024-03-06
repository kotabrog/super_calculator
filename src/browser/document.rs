use anyhow::{anyhow, Result};
use web_sys::{Document, NodeList};

use super::window;

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document Found"))
}

pub fn query_selector_all(selector: &str) -> Result<NodeList> {
    document()?.query_selector_all(selector)
        .map_err(|_| anyhow!("No elements found for selector {}", selector))
}
