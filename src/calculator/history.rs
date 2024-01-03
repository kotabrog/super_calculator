use anyhow::Result;
use crate::engine::{
    HtmlElement, Node, Element,
};
use super::{Calculator, HISTORY_ITEM};

impl Calculator {
    fn add_history_bottom(history_container: &HtmlElement, new_entry: &Node) -> Result<()> {
        let at_bottom = history_container.is_scrolled_to_bottom();
    
        history_container.append_child(&new_entry)?;
    
        if at_bottom {
            history_container.scroll_to_bottom();
        }
        Ok(())
    }

    fn add_error_entry(new_entry: &Element, value: &str, error: &str) -> Result<()> {
        let text_node = Element::new_from_tag("span")?;
        text_node.set_text_content(value);
        new_entry.append_child(&text_node.into())?;

        let error_node = Element::new_from_tag("span")?;
        error_node.set_class_name("error-text");
        error_node.set_text_content(&format!(" {}", error));
        new_entry.append_child(&error_node.into())?;
        Ok(())
    }

    pub(super) fn add_history_entry(value: &(&str, Result<String, String>), history_container: &HtmlElement) -> Result<()> {
        let new_entry = Element::new_from_tag("div")?;
        match &value.1 {
            Ok(result) => {
                new_entry.set_text_content(result);
            }
            Err(error) => {
                Self::add_error_entry(&new_entry, value.0, error)?;
            }
        }
        new_entry.set_class_name(HISTORY_ITEM);
    
        Self::add_history_bottom(&history_container, &new_entry.into())?;
        Ok(())
    }
}
