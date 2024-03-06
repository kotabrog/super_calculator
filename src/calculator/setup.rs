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
    INPUT_AREA1, INPUT_AREA2,
    variable_button::VariableButton,
    variable_manager::{VariableManager, VariableState},
};

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

        let input1 = Element::new_from_id(INPUT_AREA1)?;
        let input2 = Element::new_from_id(INPUT_AREA2)?;
        let closure = create_event_closure(move |e: web_sys::Event| {
            match Self::handle_register_variable(e) {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });
        input1.add_event_listener_with_callback("keydown", &closure)?;
        input2.add_event_listener_with_callback("keydown", &closure)?;
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

    pub fn handle_delete_button(event: web_sys::Event) -> Result<()> {
        let event = Event::new(event);
        let target = event.get_target_html_element()?;
        let parent = target.parent_element()?;
        let parent = if parent.has_class("variable-delete") {
            parent.parent_element()?
        } else {
            parent
        };
        parent.remove();
        Ok(())
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

    pub fn handle_register_variable(event: web_sys::Event) -> Result<()> {
        let event = Event::new(event);
        let event: KeyboardEvent = match event.try_into() {
            Ok(event) => event,
            Err(_) => {
                return Ok(());
            }
        };
        if event.ctrl_enter() {
            let input_area1 = HtmlInputElement::new_from_id(INPUT_AREA1)?;
            let input_area2 = HtmlInputElement::new_from_id(INPUT_AREA2)?;
            let history_container = HtmlElement::new_from_id(HISTORY_CONTAINER)?;
            let mut variable_manager = VariableManager::new_from_dom()?;
            let variable = input_area1.get_value();
            let value = input_area2.get_value();
            let result = variable_manager.validate_input(&variable, &value);
            let result = match result {
                Ok((c, num)) => {
                    let state = VariableState::new(c.to_string(), num.clone(), true);
                    variable_manager.append_variable_to_dom(&c.to_string(), state, true)?;
                    Ok(format!("{} = {}", c, num))
                }
                Err(e) => Err(e),
            };
            Self::add_history_entry(&format!("{} = {}", variable, value), &result, &history_container)?;
            input_area1.set_value("");
            input_area2.set_value("");
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
