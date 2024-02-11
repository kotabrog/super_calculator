use anyhow::Result;
use crate::browser::{
    event_to_keboard_event, get_input_element_from_event,
    get_html_select_element_from_event,
    get_html_element_from_event,
};
use super::{
    HtmlInputElement, KeyboardEvent, HtmlSelectElement,
    HtmlElement,
};

#[derive(Debug, Clone)]
pub struct Event {
    inner: web_sys::Event,
}

impl Event {
    pub fn new(inner: web_sys::Event) -> Self {
        Self { inner }
    }

    // pub fn get_target_element(&self) -> Result<Element> {
    //     Ok(Element::new(get_element_from_event(&self.inner)?))
    // }

    pub fn get_target_html_element(&self) -> Result<HtmlElement> {
        Ok(HtmlElement::new(get_html_element_from_event(&self.inner)?))
    }

    pub fn get_target_input_element(&self) -> Result<HtmlInputElement> {
        Ok(HtmlInputElement::new(get_input_element_from_event(&self.inner)?))
    }

    pub fn get_target_html_select_element(&self) -> Result<HtmlSelectElement> {
        Ok(HtmlSelectElement::new(get_html_select_element_from_event(&self.inner)?))
    }
}

impl TryInto<KeyboardEvent> for Event {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<KeyboardEvent> {
        Ok(KeyboardEvent::new(event_to_keboard_event(&self.inner)?))
    }
}
