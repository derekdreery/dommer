#[macro_use]
#[doc(hidden)]
pub mod macros;
pub mod dom_rect;
pub mod event;
pub mod node;
pub mod element;
pub mod fetch;
pub mod document;
//pub mod prelude;

pub fn document() -> document::Document {
    let window = expect_opt!(web_sys::window(), "window method failed - this is a bug!");
    let document = expect_opt!(
        window.document(),
        "calling Window::document"
    );
    document::Document::from_web_sys(document)
}
