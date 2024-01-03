use anyhow::Result;
use crate::browser::{
    create_event_closure,
    forget_event_closure,
};
use crate::engine::{
    HtmlElement, Event, KeyboardEvent, HtmlInputElement,
    Element,
};
use super::{Calculator, INPUT_AREA, FORMATTED_DISPLAY, HISTORY_CONTAINER};

impl Calculator {
    pub fn setup() -> Result<()> {
        let input = Element::new_from_id(INPUT_AREA)?;

        let closure = create_event_closure(move |e: web_sys::Event| {
            match Self::handle_input(e) {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });

        input.add_event_listener_with_callback("input", &closure)?;
        forget_event_closure(closure);
    
        let closure = create_event_closure(move |e: web_sys::Event| {
            match Self::handle_keydown(e) {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });

        input.add_event_listener_with_callback("keydown", &closure)?;
        forget_event_closure(closure);
        Ok(())
    }

    fn handle_input(event: web_sys::Event) -> Result<()> {
        let event = Event::new(event);
        let value = event
            .get_target_input_element()?
            .get_value();
        let formatted_value = Self::format_input(&value);
        let display = HtmlElement::new_from_id(FORMATTED_DISPLAY)?;
        display.set_inner_text(&formatted_value);
        Ok(())
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
            let input = HtmlInputElement::new_from_id(INPUT_AREA)?;
            let display = HtmlElement::new_from_id(FORMATTED_DISPLAY)?;
            let history_container = HtmlElement::new_from_id(HISTORY_CONTAINER)?;
            let value = display.get_inner_text();
            let value = Self::calculate_and_format(&value).unwrap_or_else(|e| e);
            Self::add_history_entry(&value, &history_container)?;
            display.set_inner_text("");
            input.set_value("");
        }
        Ok(())
    }
}
