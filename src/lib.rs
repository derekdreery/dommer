#[macro_use]
mod macros;
pub mod document;
pub mod dom_rect;
pub mod element;
pub mod event;
pub mod node;
pub mod prelude;

pub fn document() -> document::Document {
    let window = expect_opt!(web_sys::window(), "window method failed - this is a bug!");
    let document = expect_opt!(
        window.document(),
        "Window::document method failed - this is a bug!"
    );
    document::Document::from_web_sys(document)
}
