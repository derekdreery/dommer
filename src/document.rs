use crate::{element, node};

pub trait IDocument {
    fn body(&self) -> element::Element;
    fn create_element(&self, local_name: &str) -> element::Element;
    fn create_text_node(&self, data: &str) -> node::Node;
}

pub struct Document {
    inner: web_sys::Document,
}

impl Document {
    pub(crate) fn from_web_sys(inner: web_sys::Document) -> Document {
        Document { inner }
    }
}

impl IDocument for Document {
    fn body(&self) -> element::Element {
        expect_opt!(
            self.inner
                .body()
                .map(|html_el| element::Element::from_web_sys(html_el.into())),
            "Document::body returned None - this is a bug!"
        )
    }
    fn create_element(&self, local_name: &str) -> element::Element {
        expect!(
            self.inner
                .create_element(local_name)
                .map(element::Element::from_web_sys),
            "an exception was thrown calling Document::create_element - this is a bug!"
        )
    }
    fn create_text_node(&self, data: &str) -> node::Node {
            let raw = self.inner.create_text_node(data);
            node::Node::from_web_sys(raw.into())
    }
}
