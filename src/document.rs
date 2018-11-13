use crate::{element, node};

#[repr(transparent)]
pub struct Document {
    inner: web_sys::Document,
}

impl Document {
    pub(crate) fn from_web_sys(inner: web_sys::Document) -> Document {
        Document { inner }
    }

    pub fn body(&self) -> element::Element {
        expect_opt!(
            self.inner
                .body()
                .map(|html_el| element::Element::from_web_sys(html_el.into())),
            "Document::body returned None"
        )
    }
    pub fn create_element(&self, tag_name: &str) -> element::Element {
        expect!(
            self.inner
                .create_element(tag_name)
                .map(element::Element::from_web_sys),
            "calling Document::create_element"
        )
    }
    pub fn create_text_node(&self, data: &str) -> node::Node {
            let raw = self.inner.create_text_node(data);
            node::Node::from_web_sys(raw.into())
    }
}
