use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};

use super::get_element_by_id;

pub fn get_input_element_by_id(id: &str) -> Result<HtmlInputElement> {
    get_element_by_id(id)?
        .dyn_into::<HtmlInputElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlInputElement", element))
}

pub fn get_input_element_from_event(event: &Event) -> Result<HtmlInputElement> {
    event
        .target()
        .ok_or_else(|| anyhow!("No target found on event"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlInputElement", element))
}
