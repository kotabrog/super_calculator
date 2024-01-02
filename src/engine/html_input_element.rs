use anyhow::Result;
use crate::browser::get_input_element_by_id;

#[derive(Debug, Clone)]
pub struct HtmlInputElement {
    inner: web_sys::HtmlInputElement,
}

impl HtmlInputElement {
    pub fn new(inner: web_sys::HtmlInputElement) -> Self {
        Self { inner }
    }

    pub fn new_from_id(id: &str) -> Result<Self> {
        Ok(Self::new(get_input_element_by_id(id)?))
    }

    pub fn get_value(&self) -> String {
        self.inner.value()
    }

    pub fn set_value(&self, value: &str) {
        self.inner.set_value(value)
    }
}
