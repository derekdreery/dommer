use crate::{dom_rect::DomRect, node};
//use wasm_bindgen::prelude::*;

pub struct InvalidSelector;
pub struct InvalidPointerId;
pub struct InvalidCharacter;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InsertPosition {
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
}

impl InsertPosition {
    pub(crate) fn as_web_sys(&self) -> &str {
        match self {
            InsertPosition::BeforeBegin => "beforebegin",
            InsertPosition::AfterBegin => "afterbegin",
            InsertPosition::BeforeEnd => "beforeend",
            InsertPosition::AfterEnd => "afterend",
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Element {
    inner: web_sys::Element,
}

impl std::ops::Deref for Element {
    type Target = node::Node;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&web_sys::Node, &node::Node>(&*self.inner) }
    }
}

impl Element {
    pub(crate) fn from_web_sys(inner: web_sys::Element) -> Self {
        Element { inner }
    }
    pub fn closest(&self, selector: &str) -> Result<Option<Element>, InvalidSelector> {
        self.inner
            .closest(selector)
            .map(|opt| opt.map(Element::from_web_sys))
            .map_err(|_| InvalidSelector)
    }
    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.inner.get_attribute(name)
    }
    pub fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<String> {
        self.inner.get_attribute_ns(namespace, local_name)
    }
    pub fn get_bounding_client_rect(&self) -> DomRect {
        DomRect::from_web_sys(self.inner.get_bounding_client_rect())
    }
    // todo consider if this is to expensive - maybe we should wrap web_sys::DomRect and
    // provide an iterator?
    pub fn get_client_rects(&self) -> Vec<DomRect> {
        let raw = self.inner.get_client_rects();
        let length = raw.length();
        let mut out = Vec::with_capacity(length as usize);
        for i in 0..length {
            out.push(DomRect::from_web_sys(expect_opt!(
                raw.item(i),
                "out of bounds error calling DomRectList::item"
            )));
        }
        out
    }
    pub fn has_attribute(&self, name: &str) -> bool {
        self.inner.has_attribute(name)
    }
    pub fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool {
        self.inner.has_attribute_ns(namespace, local_name)
    }
    pub fn has_attributes(&self) -> bool {
        self.inner.has_attributes()
    }
    pub fn has_pointer_capture(&self, pointer_id: i32) -> bool {
        self.inner.has_pointer_capture(pointer_id)
    }

    // The following function should not error as we control the types
    pub fn insert_adjacent_element(
        &self,
        position: InsertPosition,
        element: impl Into<Element>,
    ) -> Option<Element> {
        let res = expect!(
            self.inner
                .insert_adjacent_element(position.as_web_sys(), &element.into().inner),
            "calling Element::insert_adjacent_element"
        );
        res.map(Element::from_web_sys)
    }
    // todo check the following doesn't throw
    pub fn insert_adjacent_html(&self, position: InsertPosition, data: &str) {
        expect!(
            self.inner.insert_adjacent_html(position.as_web_sys(), data),
            "calling Element::insert_adjacent_html"
        )
    }
    // todo check the following doesn't throw
    pub fn insert_adjacent_text(&self, position: InsertPosition, text: &str) {
        expect!(
            self.inner.insert_adjacent_text(position.as_web_sys(), text),
            "calling Element::insert_adjacent_text"
        )
    }
    pub fn matches(&self, selector: &str) -> Result<bool, InvalidSelector> {
        self.inner.matches(selector).map_err(|_| InvalidSelector)
    }
    // warning - this function walks the whole DOM tree even if it is called on a branch.
    pub fn query_selector(&self, selectors: &str) -> Result<Option<Element>, InvalidSelector> {
        self.inner
            .query_selector(selectors)
            .map(|opt| opt.map(Element::from_web_sys))
            .map_err(|_| InvalidSelector)
    }
    pub fn query_selector_all(&self, selectors: &str) -> Result<node::NodeList, InvalidSelector> {
        self.inner
            .query_selector_all(selectors)
            .map(node::NodeList::from_web_sys)
            .map_err(|_| InvalidSelector)
    }
    //pub fn release_capture(&self); // I think this isn't really used (point_capture)
    pub fn release_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId> {
        self.inner
            .release_pointer_capture(pointer_id)
            .map_err(|_| InvalidPointerId)
    }
    // todo mdn says this doesn't throw - is this right.
    pub fn remove_attribute(&self, name: &str) {
        expect!(
            self.inner.remove_attribute(name),
            "calling Element::remove_attribute"
        )
    }
    pub fn remove_attribute_ns(&self, namespace: Option<&str>, local_name: &str) {
        expect!(
            self.inner.remove_attribute_ns(namespace, local_name),
            "calling Element::remove_attribute_ns"
        )
    }
    // todo I tried to play with this and the console didn't show it. I don't think it can throw -
    // it fires an event on error.
    pub fn request_fullscreen(&self) {
        expect!(
            self.inner.request_fullscreen(),
            "calling Element::request_fullscreen"
        )
    }
    //pub fn request_pointer_lock(&self); // This is experimental
    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), InvalidCharacter> {
        self.inner
            .set_attribute(name, value)
            .map_err(|_| InvalidCharacter)
    }
    pub fn set_attribute_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
        value: &str,
    ) -> Result<(), InvalidCharacter> {
        self.inner
            .set_attribute_ns(namespace, local_name, value)
            .map_err(|_| InvalidCharacter)
    }
    pub fn set_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId> {
        self.inner
            .set_pointer_capture(pointer_id)
            .map_err(|_| InvalidPointerId)
    }
    // todo is `force` desirable/useful?
    pub fn toggle_attribute(&self, name: &str) -> Result<bool, InvalidCharacter> {
        self.inner
            .toggle_attribute(name)
            .map_err(|_| InvalidCharacter)
    }
    pub fn namespace_uri(&self) -> Option<String> {
        self.inner.namespace_uri()
    }
    pub fn prefix(&self) -> Option<String> {
        self.inner.prefix()
    }
    pub fn local_name(&self) -> String {
        self.inner.local_name()
    }
    pub fn tag_name(&self) -> String {
        self.inner.tag_name()
    }
    pub fn id(&self) -> String {
        self.inner.id()
    }
    pub fn set_id(&self, id: &str) {
        self.inner.set_id(id)
    }
    pub fn class_name(&self) -> String {
        self.inner.class_name()
    }
    pub fn set_class_name(&self, class_name: &str) {
        self.inner.set_class_name(class_name)
    }
    // TODO
    //pub fn class_list(&self) -> DomTokenList;
    //pub fn attributes(&self) -> NamedNodeMap;
    // todo mdn says this should be float (can be decimal)
    pub fn scroll_top(&self) -> i32 {
        self.inner.scroll_top()
    }
    pub fn set_scroll_top(&self, scroll_top: i32) {
        self.inner.set_scroll_top(scroll_top)
    }
    pub fn scroll_left(&self) -> i32 {
        self.inner.scroll_left()
    }
    pub fn set_scroll_left(&self, scroll_left: i32) {
        self.inner.set_scroll_left(scroll_left)
    }
    // getBoundingClientRect is better
    pub fn scroll_width(&self) -> i32 {
        self.inner.scroll_width()
    }
    pub fn scroll_height(&self) -> i32 {
        self.inner.scroll_height()
    }
    pub fn client_top(&self) -> i32 {
        self.inner.client_top()
    }
    pub fn client_left(&self) -> i32 {
        self.inner.client_left()
    }
    pub fn client_width(&self) -> i32 {
        self.inner.client_width()
    }
    pub fn client_height(&self) -> i32 {
        self.inner.client_height()
    }
    pub fn inner_html(&self) -> String {
        self.inner.inner_html()
    }
    pub fn set_inner_html(&self, inner_html: &str) {
        self.inner.set_inner_html(inner_html)
    }
    pub fn outer_html(&self) -> String {
        self.inner.outer_html()
    }
    pub fn set_outer_html(&self, outer_html: &str) {
        self.inner.set_outer_html(outer_html)
    }
    // todo skipping shadow dom stuff
    // skipping ChildNode, NonDocumentTypeChildNode, ParentNode::{append, prepend}
    //pub fn children(&self) -> HtmlCollection;
    pub fn first_element_child(&self) -> Option<Element> {
        self.inner.first_element_child().map(Element::from_web_sys)
    }
    pub fn last_element_child(&self) -> Option<Element> {
        self.inner.last_element_child().map(Element::from_web_sys)
    }
    pub fn child_element_count(&self) -> u32 {
        self.inner.child_element_count()
    }
}
