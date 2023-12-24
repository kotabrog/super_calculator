use anyhow::{anyhow, Result};
use web_sys::Window;

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}
