use anyhow::{anyhow, Result};
use web_sys::Document;

use super::window;

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document Found"))
}
