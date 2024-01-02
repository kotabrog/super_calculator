#[macro_use]
mod browser;

use anyhow::Result;
use wasm_bindgen::prelude::*;
use web_sys::{
    Event, HtmlElement, Node,
};

use browser::{
    get_input_element_by_id, get_html_element_by_id,
    create_event_closure, get_input_element_from_event,
    add_event_listener_with_callback, forget_event_closure,
    event_to_keboard_event, create_element, append_child,
    is_scrolled_to_bottom,
};

fn add_history_bottom(history_container: &HtmlElement, new_entry: &Node) -> Result<()> {
    let at_bottom = is_scrolled_to_bottom(&history_container);

    append_child(&history_container, &new_entry)?;

    if at_bottom {
        history_container.set_scroll_top(history_container.scroll_height());
    }
    Ok(())
}

fn add_history_entry(value: &str, history_container: &HtmlElement) -> Result<()> {
    let new_entry = create_element("div")?;
    new_entry.set_text_content(Some(value));
    new_entry.set_class_name("history-item");

    add_history_bottom(history_container, &new_entry)?;
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

fn handle_keydown(event: Event) -> Result<()> {
    let event = match event_to_keboard_event(&event) {
        Ok(event) => event,
        Err(_) => {
            return Ok(());
        }
    };
    if event.key() == "Enter" && event.ctrl_key() {
        let input_elem = get_input_element_by_id("input-area")?;
        let display = get_html_element_by_id("formatted-display")?;
        let history_container = get_html_element_by_id("history-container")?;
        let value = display.inner_text();
        let value = calculate_and_format(&value).unwrap_or_else(|e| e);
        add_history_entry(&value, &history_container)?;
        display.set_inner_text("");
        input_elem.set_value("");
    }
    Ok(())
}

pub fn format_input(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

pub fn handle_input(event: Event) -> Result<()> {
    let value = get_input_element_from_event(&event)?.value();
    let formatted_value = format_input(&value);
    let display = get_html_element_by_id("formatted-display")?;
    display.set_inner_text(&formatted_value);
    log!("input: {}", value);
    Ok(())
}

pub fn run() -> Result<()> {
    let input = get_input_element_by_id("input-area")?;

    let closure = create_event_closure(move |e: Event| {
        match handle_input(e) {
            Ok(_) => {}
            Err(e) => error!("{}", e),
        }
    });

    add_event_listener_with_callback(&input, "input", &closure)?;
    forget_event_closure(closure);

    let closure = create_event_closure(move |e: Event| {
        match handle_keydown(e) {
            Ok(_) => {}
            Err(e) => error!("{}", e),
        }
    });

    add_event_listener_with_callback(&input, "keydown", &closure)?;
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
