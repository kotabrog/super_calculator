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

    pub(super) fn add_history_entry(value: &str, history_container: &HtmlElement) -> Result<()> {
        let new_entry = Element::new_from_tag("div")?;
        new_entry.set_text_content(value);
        new_entry.set_class_name(HISTORY_ITEM);
    
        Self::add_history_bottom(&history_container, &new_entry.into())?;
        Ok(())
    }
}
