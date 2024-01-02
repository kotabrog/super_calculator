#[macro_use]
mod browser;
mod engine;

use anyhow::Result;
use wasm_bindgen::prelude::*;

use browser::{
    create_event_closure,
    forget_event_closure,
};

use engine::{
    Element, Event, KeyboardEvent, HtmlInputElement,
    HtmlElement, Node,
};

fn add_history_bottom(history_container: &HtmlElement, new_entry: &Node) -> Result<()> {
    let at_bottom = history_container.is_scrolled_to_bottom();

    history_container.append_child(&new_entry)?;

    if at_bottom {
        history_container.scroll_to_bottom();
    }
    Ok(())
}

fn add_history_entry(value: &str, history_container: &HtmlElement) -> Result<()> {
    let new_entry = Element::new_from_tag("div")?;
    new_entry.set_text_content(value);
    new_entry.set_class_name("history-item");

    add_history_bottom(&history_container, &new_entry.into())?;
    Ok(())
}

fn calculate_and_format(input: &str) -> Result<String, String> {
    if let Some(plus_index) = input.find('+') {
        let (left, right) = input.split_at(plus_index);
        let x = left.trim().parse::<i32>().map_err(|_| "左側の数値の解析に失敗しました")?;
        let y = right[1..].trim().parse::<i32>().map_err(|_| "右側の数値の解析に失敗しました")?;

        Ok(format!("{} + {} → {}", x, y, x + y))
    } else {
        Err("式に '+' が含まれていません".to_string())
    }
}

fn handle_keydown(event: web_sys::Event) -> Result<()> {
    let event = Event::new(event);
    let event: KeyboardEvent = match event.try_into() {
        Ok(event) => event,
        Err(_) => {
            return Ok(());
        }
    };
    if event.ctrl_enter() {
        let input = HtmlInputElement::new_from_id("input-area")?;
        let display = HtmlElement::new_from_id("formatted-display")?;
        let history_container = HtmlElement::new_from_id("history-container")?;
        let value = display.get_inner_text();
        let value = calculate_and_format(&value).unwrap_or_else(|e| e);
        add_history_entry(&value, &history_container)?;
        display.set_inner_text("");
        input.set_value("");
    }
    Ok(())
}

pub fn format_input(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

pub fn handle_input(event: web_sys::Event) -> Result<()> {
    let event = Event::new(event);
    let value = event
        .get_target_input_element()?
        .get_value();
    let formatted_value = format_input(&value);
    let display = HtmlElement::new_from_id("formatted-display")?;
    display.set_inner_text(&formatted_value);
    log!("input: {}", value);
    Ok(())
}

pub fn run() -> Result<()> {
    let input = Element::new_from_id("input-area")?;

    let closure = create_event_closure(move |e: web_sys::Event| {
        match handle_input(e) {
            Ok(_) => {}
            Err(e) => error!("{}", e),
        }
    });

    input.add_event_listener_with_callback("input", &closure)?;
    forget_event_closure(closure);

    let closure = create_event_closure(move |e: web_sys::Event| {
        match handle_keydown(e) {
            Ok(_) => {}
            Err(e) => error!("{}", e),
        }
    });

    input.add_event_listener_with_callback("keydown", &closure)?;
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
