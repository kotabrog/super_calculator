mod window;
mod document;
mod element;
mod html_element;
mod input_element;
mod closure;
mod event;

pub use window::window;
pub use document::document;
pub use element::{
    get_element_by_id, create_element, element_append_child,
};
pub use html_element::{
    get_html_element_by_id, append_child, is_scrolled_to_bottom,
};
pub use input_element::{
    get_input_element_by_id, get_input_element_from_event,
};
pub use closure::{
    create_event_closure, EventClosure, add_event_listener_with_callback,
    forget_event_closure,
};
pub use event::event_to_keboard_event;

#[allow(unused_macros)]
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into())
    };
}
