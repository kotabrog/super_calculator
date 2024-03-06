use anyhow::Result;
use crate::browser::{
    get_element_by_id, create_element,
    add_event_listener_with_callback,
    element_append_child, crate_element_from_node,
    element_query_selector, element_text_content,
    has_class_element, get_html_element_from_element,
    EventClosure,
};
use super::{Node, HtmlElement};

#[derive(Debug, Clone)]
pub struct Element {
    inner: web_sys::Element,
}

impl Element {
    pub fn new(inner: web_sys::Element) -> Self {
        Self { inner }
    }

    pub fn new_from_id(id: &str) -> Result<Self> {
        Ok(Self::new(get_element_by_id(id)?))
    }

    pub fn new_from_tag(tag: &str) -> Result<Self> {
        Ok(Self::new(create_element(tag)?))
    }

    pub fn new_from_node(node: &Node) -> Result<Self> {
        Ok(Self::new(crate_element_from_node(node.inner().clone())?))
    }

    pub fn convert_to_html_element(&self) -> Result<HtmlElement> {
        Ok(HtmlElement::new(get_html_element_from_element(self.inner.clone())?))
    }

    pub fn text_content(&self) -> Result<String> {
        element_text_content(&self.inner)
    }

    pub fn set_text_content(&self, value: &str) {
        self.inner.set_text_content(Some(value))
    }

    pub fn has_class(&self, value: &str) -> bool {
        has_class_element(&self.inner, value)
    }

    pub fn set_class_name(&self, value: &str) {
        self.inner.set_class_name(value)
    }

    pub fn append_child(&self, child: &Node) -> Result<Node> {
        Ok(Node::new(element_append_child(
            &self.inner, child.inner())?))
    }

    pub fn search_child_by_selector(&self, selector: &str) -> Result<Element> {
        Ok(Element::new(element_query_selector(
            &self.inner, selector)?))
    }

    pub fn add_event_listener_with_callback(
        &self,
        event: &str,
        callback: &EventClosure,
    ) -> Result<()> {
        add_event_listener_with_callback(
            &self.inner,
            event,
            callback,
        )
    }
}

impl Into<Node> for Element {
    fn into(self) -> Node {
        Node::new(self.inner.into())
    }
}
