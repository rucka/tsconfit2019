use wasm_bindgen::prelude::*;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn timestamp() -> f64 {
    js_sys::Date::now()
}

#[wasm_bindgen]
pub fn main() {
    utils::set_panic_hook();
    log("Hello, rust-wasm!");
    runner::run(&|message| log(message), &timestamp);
}
