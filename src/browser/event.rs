use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

pub fn event_to_keboard_event(event: &Event) -> Result<KeyboardEvent> {
    event
        .dyn_ref::<KeyboardEvent>()
        .ok_or_else(|| anyhow!("No KeyboardEvent found on event"))
        .map(|event| event.clone())
}
