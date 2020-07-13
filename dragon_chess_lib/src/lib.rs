extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

mod grid;
mod tests;
mod player;
mod board;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

mod pieces;






#[wasm_bindgen]
pub fn return_string() -> String {
    "hello".into()
}

#[wasm_bindgen]
pub fn forward() -> js_sys::Array {
    js_sys::Array::new()
}
