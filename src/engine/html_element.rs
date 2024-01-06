use anyhow::Result;
use crate::browser::{
    get_html_element_by_id, is_scrolled_to_bottom, append_child,
    remove_class, set_class,
};
use super::Node;

#[derive(Debug, Clone)]
pub struct HtmlElement {
    inner: web_sys::HtmlElement,
}

impl HtmlElement {
    pub fn new(inner: web_sys::HtmlElement) -> Self {
        Self { inner }
    }

    pub fn new_from_id(id: &str) -> Result<Self> {
        Ok(Self::new(get_html_element_by_id(id)?))
    }

    pub fn get_inner_text(&self) -> String {
        self.inner.inner_text()
    }

    pub fn set_inner_text(&self, value: &str) {
        self.inner.set_inner_text(value)
    }

    pub fn set_class(&self, class: &str) {
        set_class(&self.inner, class)
    }

    pub fn remove_class(&self, class: &str) -> Result<()> {
        remove_class(&self.inner, class)
    }

    pub fn append_child(&self, child: &Node) -> Result<Node> {
        Ok(Node::new(append_child(
            &self.inner, child.inner())?))
    }

    pub fn is_scrolled_to_bottom(&self) -> bool {
        is_scrolled_to_bottom(&self.inner)
    }

    pub fn scroll_to_bottom(&self) {
        self.inner.set_scroll_top(self.inner.scroll_height())
    }
}
