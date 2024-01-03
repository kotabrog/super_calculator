#[macro_use]
mod browser;
mod engine;
mod calculator;

use anyhow::Result;
use wasm_bindgen::prelude::*;

use calculator::Calculator;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    match Calculator::setup() {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };
    Ok(())
}
