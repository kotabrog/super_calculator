use anyhow::{anyhow, Result};
use web_sys::Element;

use super::document;

pub fn get_element_by_id(id: &str) -> Result<Element> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("No Element found with ID {}", id))
}
