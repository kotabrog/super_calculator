use anyhow::{Result, anyhow};
use std::collections::HashMap;
use crate::engine::{NodeList, Element, HtmlElement};
use crate::browser::{create_event_closure, forget_event_closure};
use super::num::Num;
use super::{Calculator, LEFT_PANEL};

#[derive(Debug, Clone)]
pub struct VariableState {
    pub name: String,
    pub value: Num,
    pub active: bool,
}

impl VariableState {
    pub fn new(name: String, value: Num, active: bool) -> Self {
        Self {
            name,
            value,
            active,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableManager {
    variables: HashMap<String, VariableState>,
}

impl VariableManager {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    // pub fn get(&self, name: &str) -> Option<&VariableState> {
    //     self.variables.get(name)
    // }

    pub fn contain(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn set(&mut self, name: &str, state: VariableState) {
        self.variables.insert(name.to_string(), state);
    }

    // pub fn remove(&mut self, name: &str) {
    //     self.variables.remove(name);
    // }

    pub fn new_from_dom() -> Result<Self> {
        let mut manager = Self::new();
        let node_list = NodeList::new_from_selector(".variable-item")?;
        for i in 0..node_list.length() {
            let node = node_list.get(i)?;
            let element = Element::new_from_node(&node)?;
            let active = element.has_class("active");
            let name_element = element.search_child_by_selector(".variable-name")?;
            let name = name_element.text_content()?;
            let value_element = element.search_child_by_selector(".variable-value")?;
            let value = value_element.text_content()?;
            let (variable, value) = manager.validate_input(&name, &value)
                .expect(&format!("Failed to validate input: {} {}", name, value));
            let state = VariableState {
                name: variable.to_string(),
                value,
                active,
            };
            manager.set(&name, state);
        }
        Ok(manager)
    }

    fn search_item_from_dom(&self, name: &str) -> Result<Element> {
        let node_list = NodeList::new_from_selector(".variable-item")?;
        for i in 0..node_list.length() {
            let node = node_list.get(i)?;
            let element = Element::new_from_node(&node)?;
            let name_element = element.search_child_by_selector(".variable-name")?;
            let node_name = name_element.text_content()?;
            if name == node_name {
                return Ok(element);
            }
        }
        Err(anyhow!("Failed to find item"))
    }

    fn add_event_to_element(&self, element: &Element) -> Result<()> {
        let closure = create_event_closure(move |e: web_sys::Event| {
            match Calculator::handle_toggle_activate(e) {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });
        element.add_event_listener_with_callback("click", &closure)?;
        forget_event_closure(closure);

        let delete_button = element.search_child_by_selector(".variable-delete")?;
        let closure = create_event_closure(move |e: web_sys::Event| {
            match Calculator::handle_delete_button(e) {
                Ok(_) => {}
                Err(e) => error!("{}", e),
            }
        });
        delete_button.add_event_listener_with_callback("click", &closure)?;
        forget_event_closure(closure);
        Ok(())
    }


    fn get_add_variable_html(&self, variable: &String, value: &Num) -> String {
        format!("<div class=\"variable-content\">
    <span class=\"variable-name\">{}</span>
    <span class=\"variable-equal\">=</span>
    <span class=\"variable-value\">{}</span>
</div>
<div class=\"variable-delete\">
    <span class=\"material-symbols-outlined\">
        delete
    </span>
</div>", variable, value)
    }

    pub fn append_variable_to_dom(&mut self, name: &str, state: VariableState, active: bool) -> Result<()> {
        let left_panel = HtmlElement::new_from_id(LEFT_PANEL)?;
        if self.contain(name) {
            self.set(name, state.clone());
            let element = self.search_item_from_dom(name)?;
            let element = element.search_child_by_selector(".variable-value")?;
            element.set_text_content(state.value.to_string().as_str());
        } else {
            self.set(name, state.clone());
            let element = Element::new_from_tag("div")?;
            element.set_class_name("variable-item");
            let element = element.convert_to_html_element()?;
            if active {
                element.add_class("active")?;
            }
            let html_string = self.get_add_variable_html(&state.name, &state.value);
            element.set_inner_html(&html_string);
            let element = element.convert_to_element()?;
            self.add_event_to_element(&element)?;
            left_panel.append_child(&element.into())?;
        }
        Ok(())
    }

    pub fn validate_variable(&self, name: &str) -> Result<char, String> {
        if name.is_empty() {
            return Err("変数が空です".to_string());
        }
        if name.len() != 1 {
            return Err("変数は1文字でなければなりません".to_string());
        }
        let c = name.chars().next().unwrap();
        if !c.is_ascii_lowercase() {
            return Err("変数はa-zでなければなりません".to_string());
        }
        Ok(c)
    }

    pub fn validate_value(&self, value: &str) -> Result<Num, String> {
        if value.is_empty() {
            return Err("変数の値が空です".to_string());
        }
        let value = Num::parse(value)?;
        Ok(value)
    }

    pub fn validate_input(&self, name: &str, value: &str) -> Result<(char, Num), String> {
        let c = self.validate_variable(name)?;
        let value = self.validate_value(value)?;
        Ok((c, value))
    }
}
