use wasm_bindgen::prelude::*;

// This function will be callable from JavaScript
#[wasm_bindgen]
pub fn reo_greet(name: &str) -> String {
    format!("Kia ora, {}! Welcome to ReO.", name)
}
