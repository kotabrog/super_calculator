use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, Event};

// pub fn get_html_select_element_by_id(id: &str) -> Result<HtmlSelectElement> {
//     get_element_by_id(id)?
//         .dyn_into::<HtmlSelectElement>()
//         .map_err(|element| anyhow!("Error converting {:#?} to HtmlSelectElement", element))
// }

pub fn get_html_select_element_from_event(event: &Event) -> Result<HtmlSelectElement> {
    event
        .target()
        .ok_or_else(|| anyhow!("No target found on event"))?
        .dyn_into::<HtmlSelectElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlSelectElement", element))
}
