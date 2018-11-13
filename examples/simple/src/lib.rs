extern crate dommer;

use wasm_bindgen::prelude::*;
use dommer::prelude::*;

#[wasm_bindgen]
pub extern "C" fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let document = dommer::document();
    console_web::println!("Hello world");
    let container = document.create_element("div");
    let decrement_btn = document.create_element("button");
    decrement_btn.set_text_content(Some("<-"));
    let count_text = document.create_text_node("0");
    let increment_btn = document.create_element("button");
    increment_btn.set_text_content(Some("->"));
    container.append_child(decrement_btn.clone());
    container.append_child(count_text.clone());
    container.append_child(increment_btn.clone());
    document.body().append_child(container)
}
