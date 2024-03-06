use anyhow::Result;
use crate::browser::{
    get_html_element_by_id, is_scrolled_to_bottom, append_child,
    remove_class, set_class, has_class, add_class, parent_element,
    remove, set_inner_html,
    create_element_from_html_element,
};
use super::{Node, Element};

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

    // pub fn convert_to_node(&self) -> Result<Node> {
    //     Ok(Node::new(get_node_from_html_element(self.inner.clone())?))
    // }

    pub fn convert_to_element(&self) -> Result<Element> {
        Ok(Element::new(create_element_from_html_element(self.inner.clone())?))
    }

    pub fn remove(&self) {
        remove(&self.inner)
    }

    pub fn get_inner_text(&self) -> String {
        self.inner.inner_text()
    }

    pub fn set_inner_text(&self, value: &str) {
        self.inner.set_inner_text(value)
    }

    pub fn set_inner_html(&self, value: &str) {
        set_inner_html(&self.inner, value)
    }

    pub fn has_class(&self, class: &str) -> bool {
        has_class(&self.inner, class)
    }

    pub fn add_class(&self, class: &str) -> Result<()> {
        add_class(&self.inner, class)
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

    pub fn parent_element(&self) -> Result<Self> {
        Ok(Self::new(parent_element(&self.inner)?))
    }

    pub fn is_scrolled_to_bottom(&self) -> bool {
        is_scrolled_to_bottom(&self.inner)
    }

    pub fn scroll_to_bottom(&self) {
        self.inner.set_scroll_top(self.inner.scroll_height())
    }
}
