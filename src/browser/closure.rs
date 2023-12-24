use anyhow::{anyhow, Result};
use wasm_bindgen::{
    JsCast,
    closure::{Closure, WasmClosure}
};
use web_sys::{Event, EventTarget};

pub type EventClosure = Closure<dyn FnMut(Event)>;

fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}

pub fn create_event_closure<F>(f: F) -> EventClosure
where
    F: FnMut(Event) + 'static,
{
    closure_wrap(Box::new(f))
}

pub fn add_event_listener_with_callback(
    element: &EventTarget,
    event_name: &str,
    callback: &EventClosure,
) -> Result<()> {
    element
        .add_event_listener_with_callback(
            event_name,
            callback.as_ref().unchecked_ref())
        .map_err(|e| anyhow!("Error adding event listener: {:?}", e))
}

pub fn forget_event_closure(closure: EventClosure) {
    closure.forget();
}
