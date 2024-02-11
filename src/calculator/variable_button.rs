use anyhow::Result;
use crate::engine::HtmlElement;

#[derive(Debug, Clone)]
pub struct VariableButton {
    inner: HtmlElement,
}

impl VariableButton {
    pub fn new(inner: HtmlElement) -> Self {
        Self { inner }
    }

    pub fn is_active(&self) -> bool {
        self.inner.has_class("active")
    }

    pub fn add_active(&self) -> Result<()> {
        self.inner.add_class("active")
    }

    pub fn remove_active(&self) -> Result<()> {
        self.inner.remove_class("active")
    }

    pub fn toggle_active(&self) -> Result<()> {
        if self.is_active() {
            self.remove_active()
        } else {
            self.add_active()
        }
    }
}
