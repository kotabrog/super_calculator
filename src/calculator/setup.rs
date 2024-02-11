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
    HELP_POPUP_CONTAINER, HIDDEN, HELP_BUTTON, CLOSE_HELP, MODE_SELECT,
    CALCULATION_INPUT, VARIABLE_ASSIGNMENT_INPUT,
    variable_button::VariableButton,
};

use wasm_bindgen::JsCast;

enum Mode {
    Calculation,
    VariableAssignment,
}

impl Mode {
    fn str_to_mode(value: &str) -> Result<Self> {
        match value {
            "calculation" => Ok(Self::Calculation),
            "variable-assignment" => Ok(Self::VariableAssignment),
            _ => Err(anyhow::anyhow!("Unknown mode")),
        }
    }
}

impl Calculator {
    pub fn setup() -> Result<()> {
        let mode_select = Element::new_from_id(MODE_SELECT)?;
        let closure = create_event_closure(move |e: web_sys::Event| {
            match Self::handle_switch_mode(e) {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });
        mode_select.add_event_listener_with_callback("change", &closure)?;
        forget_event_closure(closure);

        // temp code
        let document = crate::browser::document()?;
        let variable_items = document.query_selector_all(".variable-item")
            .map_err(|_| anyhow::anyhow!("No variable items found"))?;
        for i in 0..variable_items.length() {
            let item = variable_items.get(i)
                .ok_or_else(|| anyhow::anyhow!("No item found at index {}", i))?
                .dyn_into::<web_sys::Element>()
                .map_err(|value| anyhow::anyhow!("Error converting {:#?} to HtmlElement", value))?;
            let item = Element::new(item);
            let closure = create_event_closure(move |e: web_sys::Event| {
                match Self::handle_toggle_activate(e) {
                    Ok(_) => {}
                    Err(e) => error!("{}", e),
                }
            });
            item.add_event_listener_with_callback("click", &closure)?;
            forget_event_closure(closure);
        }

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

    pub fn handle_toggle_activate(event: web_sys::Event) -> Result<()> {
        let event = Event::new(event);
        let target = event.get_target_html_element()?;
        let button = VariableButton::new(target);
        button.toggle_active()
    }

    fn handle_switch_mode(event: web_sys::Event) -> Result<()> {
        let event = Event::new(event);
        let mode_select = event.get_target_html_select_element()?;
        let mode = mode_select.get_value();
        let mode = Mode::str_to_mode(&mode)?;
        let calc_input = HtmlElement::new_from_id(CALCULATION_INPUT)?;
        let var_input = HtmlElement::new_from_id(VARIABLE_ASSIGNMENT_INPUT)?;
        match mode {
            Mode::Calculation => {
                calc_input.remove_class(HIDDEN)?;
                var_input.set_class(HIDDEN);
            }
            Mode::VariableAssignment => {
                calc_input.set_class(HIDDEN);
                var_input.remove_class(HIDDEN)?;
            }
        }
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
