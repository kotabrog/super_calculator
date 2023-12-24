#[macro_use]
mod browser;

use anyhow::Result;
use wasm_bindgen::prelude::*;
use web_sys::Event;

use browser::{
    get_input_element_by_id, get_html_element_by_id,
    create_event_closure, get_input_element_from_event,
    add_event_listener_with_callback, forget_event_closure,
};

pub fn input_closure(e: Event) -> Result<()> {
        let value = get_input_element_from_event(&e)?.value();
        let formatted_value = format_input(&value);
        let display = get_html_element_by_id("formatted-display")?;
        display.set_inner_text(&formatted_value);
        log!("input: {}", value);
        Ok(())
}

pub fn run() -> Result<()> {
    let input = get_input_element_by_id("input-area")?;

    let closure = create_event_closure(move |e: Event| {
        match input_closure(e) {
            Ok(_) => {}
            Err(e) => error!("{}", e),
        }
    });

    add_event_listener_with_callback(&input, "input", &closure)?;
    forget_event_closure(closure);
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    match run() {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };
    Ok(())
}

pub fn format_input(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}
