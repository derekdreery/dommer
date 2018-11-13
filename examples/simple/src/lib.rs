extern crate dommer;

use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use dommer::node::Node;
use dommer::event::{SubscribeGuard, EventKind};

#[wasm_bindgen]
pub struct App {
    count: Rc<RefCell<u32>>,
    guards: Vec<SubscribeGuard>,
}

#[wasm_bindgen]
impl App {
    /// You can use this to destroy all the memory associated with this object, leaving static
    /// html.
    #[wasm_bindgen]
    pub extern fn shutdown(self) {
        dommer::println!("At end, count was {}", self.count.borrow());
        let _guards = self.guards;
    }
}

#[wasm_bindgen]
pub extern fn run() -> App {
    let count = Rc::new(RefCell::new(0u32));

    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        dommer::eprintln!("{}", info.to_string());
    }));
    let document = dommer::document();
    let container = document.create_element("div");
    let decrement_btn = document.create_element("button");
    decrement_btn.set_text_content(Some("<-"));
    let count_text = Rc::new(document.create_text_node(&format!("{}", count.borrow())));
    let increment_btn = document.create_element("button");
    increment_btn.set_text_content(Some("->"));
    container.append_child(&decrement_btn);
    container.append_child(&count_text);
    container.append_child(&increment_btn);
    document.body().append_child(&container);

    let mut guards = Vec::new();
    let dec_count = count.clone();
    let dec_count_text = count_text.clone();
    guards.push(decrement_btn.add_event_listener(EventKind::Click, move |_| {
        if *dec_count.borrow() == 0 {
            return;
        }
        *dec_count.borrow_mut() -= 1;
        update_count(*dec_count.borrow(), &dec_count_text);
    }));
    let inc_count = count.clone();
    let inc_count_text = count_text.clone();
    guards.push(increment_btn.add_event_listener(EventKind::Click, move |_| {
        use std::u32;
        if *inc_count.borrow() == u32::MAX {
            return;
        }
        *inc_count.borrow_mut() += 1;
        update_count(*inc_count.borrow(), &inc_count_text);
    }));
    App {
        count,
        guards,
    }
}

fn update_count(new_count: u32, text_node: &Node) {
    text_node.set_text_content(Some(&format!("{}", new_count)));
}
