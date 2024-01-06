use anyhow::Result;
use crate::browser::{
    create_event_closure,
    forget_event_closure,
};
use crate::engine::{
    HtmlElement, Event, KeyboardEvent, HtmlInputElement,
    Element,
};
use super::{
    Calculator, INPUT_AREA, FORMATTED_DISPLAY, HISTORY_CONTAINER,
    HELP_POPUP_CONTAINER, HIDDEN, HELP_BUTTON, CLOSE_HELP,
};

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

        let help_button = Element::new_from_id(HELP_BUTTON)?;
        let closure = create_event_closure(move |_: web_sys::Event| {
            match Self::handle_help_popup() {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });

        help_button.add_event_listener_with_callback("click", &closure)?;
        forget_event_closure(closure);

        let close_help = Element::new_from_id(CLOSE_HELP)?;
        let closure = create_event_closure(move |_: web_sys::Event| {
            match Self::handle_popup_close() {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });
        close_help.add_event_listener_with_callback("click", &closure)?;
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
            let result = Self::calculate_and_format(&value);
            Self::add_history_entry(&value, &result, &history_container)?;
            display.set_inner_text("");
            input.set_value("");
        }
        Ok(())
    }

    fn handle_help_popup() -> Result<()> {
        let help_popup = HtmlElement::new_from_id(HELP_POPUP_CONTAINER)?;
        help_popup.remove_class(HIDDEN)?;
        Ok(())
    }

    fn handle_popup_close() -> Result<()> {
        let help_popup = HtmlElement::new_from_id(HELP_POPUP_CONTAINER)?;
        help_popup.set_class(HIDDEN);
        Ok(())
    }
}
