use crate::{dom_rect::DomRect, event, node};
use wasm_bindgen::prelude::*;

pub struct InvalidSelector;
pub struct InvalidPointerId;
pub struct InvalidCharacter;

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

pub trait IElement {
    fn closest(&self, selector: &str) -> Result<Option<Element>, InvalidSelector>;
    fn get_attribute(&self, name: &str) -> Option<String>;
    fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<String>;
    fn get_bounding_client_rect(&self) -> DomRect;
    fn get_client_rects(&self) -> Vec<DomRect>;
    fn has_attribute(&self, name: &str) -> bool;
    fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool;
    fn has_attributes(&self) -> bool;
    fn has_pointer_capture(&self, pointer_id: i32) -> bool;
    // The following function should not error as we control the types
    fn insert_adjacent_element(
        &self,
        position: InsertPosition,
        element: impl Into<Element>,
    ) -> Option<Element>;
    // todo check the following doesn't throw
    fn insert_adjacent_html(&self, position: InsertPosition, data: &str);
    // todo check the following doesn't throw
    fn insert_adjacent_text(&self, position: InsertPosition, text: &str);
    fn matches(&self, selector: &str) -> Result<bool, InvalidSelector>;
    // warning - this function walks the whole DOM tree even if it is called on a branch.
    fn query_selector(&self, selectors: &str) -> Result<Option<Element>, InvalidSelector>;
    fn query_selector_all(&self, selectors: &str) -> Result<node::NodeList, InvalidSelector>;
    //fn release_capture(&self); // I think this isn't really used (point_capture)
    fn release_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId>;
    // todo mdn says this doesn't throw - is this right.
    fn remove_attribute(&self, name: &str);
    fn remove_attribute_ns(&self, namespace: Option<&str>, local_name: &str);
    // todo I tried to play with this and the console didn't show it. I don't think it can throw -
    // it fires an event on error.
    fn request_fullscreen(&self);
    //fn request_pointer_lock(&self); // This is experimental
    fn set_attribute(&self, name: &str, value: &str) -> Result<(), InvalidCharacter>;
    fn set_attribute_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
        value: &str,
    ) -> Result<(), InvalidCharacter>;
    fn set_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId>;
    // todo is `force` desirable/useful?
    fn toggle_attribute(&self, name: &str) -> Result<bool, InvalidCharacter>;
    fn namespace_uri(&self) -> Option<String>;
    fn prefix(&self) -> Option<String>;
    fn local_name(&self) -> String;
    fn tag_name(&self) -> String;
    fn id(&self) -> String;
    fn set_id(&self, id: &str);
    fn class_name(&self) -> String;
    fn set_class_name(&self, class_name: &str);
    // todo
    //fn class_list(&self) -> DomTokenList;
    //fn attributes(&self) -> NamedNodeMap;
    // todo mdn says this should be float (can be decimal)
    fn scroll_top(&self) -> i32;
    fn set_scroll_top(&self, scroll_top: i32);
    fn scroll_left(&self) -> i32;
    fn set_scroll_left(&self, scroll_left: i32);
    // getBoundingClientRect is better
    fn scroll_width(&self) -> i32;
    fn scroll_height(&self) -> i32;
    fn client_top(&self) -> i32;
    fn client_left(&self) -> i32;
    fn client_width(&self) -> i32;
    fn client_height(&self) -> i32;
    fn inner_html(&self) -> String;
    fn set_inner_html(&self, inner_html: &str);
    fn outer_html(&self) -> String;
    fn set_outer_html(&self, outer_html: &str);
    // todo skipping shadow dom stuff
    // skipping ChildNode, NonDocumentTypeChildNode, ParentNode::{append, prepend}
    //fn children(&self) -> HtmlCollection;
    fn first_element_child(&self) -> Option<Element>;
    fn last_element_child(&self) -> Option<Element>;
    fn child_element_count(&self) -> u32;
}

#[derive(Debug, Clone)]
pub struct Element {
    inner: web_sys::Element,
}

impl Element {
    pub(crate) fn from_web_sys(inner: web_sys::Element) -> Self {
        Element { inner }
    }
}

impl From<Element> for node::Node {
    fn from(element: Element) -> node::Node {
        node::Node {
            inner: element.inner.into(),
        }
    }
}

impl From<Element> for event::EventTarget {
    fn from(element: Element) -> event::EventTarget {
        node::Node::from(element).into()
    }
}

impl event::IEventTarget for Element {
    fn add_event_listener_opts(
        &self,
        event_kind: event::EventKind,
        listener: impl Fn(event::Event) + 'static,
        options: event::AddEventListenerOptions,
    ) -> event::SubscribeGuard {
        event::EventTarget::from(self.to_owned())
            .add_event_listener_opts(event_kind, listener, options)
    }
    fn dispatch_event(&self, event: impl Into<event::Event>) -> bool {
        event::EventTarget::from(self.to_owned()).dispatch_event(event)
    }
}

impl node::INode for Element {
    fn append_child(&self, node: impl AsRef<node::Node>) {
        let node: &node::Node = node.as_ref();
        let this: &web_sys::Node = self.inner.as_ref();
        expect!(
            this.append_child(&node.inner),
            "unexpected failure of Node::append_child - this is a bug!"
        );
    }
    fn clone_node(&self, deep: bool) -> node::Node {
        node::Node::from(self.to_owned()).clone_node(deep)
    }
    fn compare_document_position(&self, other: impl Into<node::Node>) -> node::DocumentPosition {
        node::Node::from(self.to_owned()).compare_document_position(other)
    }
    fn contains(&self, other: impl Into<node::Node>) -> bool {
        node::Node::from(self.to_owned()).contains(other)
    }
    fn get_root_node(&self) -> node::Node {
        node::Node::from(self.to_owned()).get_root_node()
    }
    fn has_child_nodes(&self) -> bool {
        node::Node::from(self.to_owned()).has_child_nodes()
    }
    fn insert_before(&self, node: impl Into<node::Node>, child: Option<impl Into<node::Node>>) {
        node::Node::from(self.to_owned()).insert_before(node, child)
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        node::Node::from(self.to_owned()).is_default_namespace(namespace)
    }
    fn is_equal_node(&self, node: impl Into<node::Node>) -> bool {
        node::Node::from(self.to_owned()).is_equal_node(node)
    }
    fn is_same_node(&self, node: impl Into<node::Node>) -> bool {
        node::Node::from(self.to_owned()).is_same_node(node)
    }
    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<String> {
        node::Node::from(self.to_owned()).lookup_namespace_uri(prefix)
    }
    fn lookup_prefix(&self, namespace: Option<&str>) -> Option<String> {
        node::Node::from(self.to_owned()).lookup_prefix(namespace)
    }
    fn normalize(&self) {
        node::Node::from(self.to_owned()).normalize()
    }
    fn remove_child(&self, child: impl Into<node::Node>) -> Result<node::Node, JsValue> {
        node::Node::from(self.to_owned()).remove_child(child)
    }
    fn replace_child(
        &self,
        node: impl Into<node::Node>,
        child: impl Into<node::Node>,
    ) -> Result<node::Node, JsValue> {
        node::Node::from(self.to_owned()).replace_child(node, child)
    }
    fn node_type(&self) -> node::NodeType {
        node::Node::from(self.to_owned()).node_type()
    }
    fn node_name(&self) -> String {
        node::Node::from(self.to_owned()).node_name()
    }
    fn base_uri(&self) -> Result<Option<String>, JsValue> {
        node::Node::from(self.to_owned()).base_uri()
    }
    fn is_connected(&self) -> bool {
        node::Node::from(self.to_owned()).is_connected()
    }
    //fn owner_document(&self) -> Option<Document> {
    //    unimplemented!()
    //}
    fn parent_node(&self) -> Option<node::Node> {
        node::Node::from(self.to_owned()).parent_node()
    }
    fn parent_element(&self) -> Option<Element> {
        node::Node::from(self.to_owned()).parent_element()
    }
    fn first_child(&self) -> Option<node::Node> {
        node::Node::from(self.to_owned()).first_child()
    }
    fn last_child(&self) -> Option<node::Node> {
        node::Node::from(self.to_owned()).last_child()
    }
    fn previous_sibling(&self) -> Option<node::Node> {
        node::Node::from(self.to_owned()).previous_sibling()
    }
    fn next_sibling(&self) -> Option<node::Node> {
        node::Node::from(self.to_owned()).next_sibling()
    }
    fn node_value(&self) -> Option<String> {
        node::Node::from(self.to_owned()).node_value()
    }
    fn set_node_value(&self, node_value: Option<&str>) {
        node::Node::from(self.to_owned()).set_node_value(node_value)
    }
    fn text_content(&self) -> Option<String> {
        node::Node::from(self.to_owned()).text_content()
    }
    fn set_text_content(&self, text_content: Option<&str>) {
        node::Node::from(self.to_owned()).set_text_content(text_content)
    }
}

impl IElement for Element {
    fn closest(&self, selector: &str) -> Result<Option<Element>, InvalidSelector> {
        self.inner
            .closest(selector)
            .map(|opt| opt.map(Element::from_web_sys))
            .map_err(|_| InvalidSelector)
    }
    fn get_attribute(&self, name: &str) -> Option<String> {
        self.inner.get_attribute(name)
    }
    fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<String> {
        self.inner.get_attribute_ns(namespace, local_name)
    }
    fn get_bounding_client_rect(&self) -> DomRect {
        DomRect::from_web_sys(self.inner.get_bounding_client_rect())
    }
    // todo consider if this is to expensive - maybe we should wrap web_sys::DomRect and
    // provide an iterator?
    fn get_client_rects(&self) -> Vec<DomRect> {
        let raw = self.inner.get_client_rects();
        let length = raw.length();
        let mut out = Vec::with_capacity(length as usize);
        for i in 0..length {
            out.push(DomRect::from_web_sys(expect_opt!(
                raw.item(i),
                "error calling DomRectList::item - this is a bug!"
            )));
        }
        out
    }
    fn has_attribute(&self, name: &str) -> bool {
        self.inner.has_attribute(name)
    }
    fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool {
        self.inner.has_attribute_ns(namespace, local_name)
    }
    fn has_attributes(&self) -> bool {
        self.inner.has_attributes()
    }
    fn has_pointer_capture(&self, pointer_id: i32) -> bool {
        self.inner.has_pointer_capture(pointer_id)
    }

    fn insert_adjacent_element(
        &self,
        position: InsertPosition,
        element: impl Into<Element>,
    ) -> Option<Element> {
        let res = expect!(
            self.inner
                .insert_adjacent_element(position.as_web_sys(), &element.into().inner),
            "error calling Element::insert_adjacent_element - this is a bug!"
        );
        res.map(Element::from_web_sys)
    }
    fn insert_adjacent_html(&self, position: InsertPosition, data: &str) {
        expect!(
            self.inner.insert_adjacent_html(position.as_web_sys(), data),
            "error calling Element::insert_adjacent_html - this is a bug!"
        )
    }
    fn insert_adjacent_text(&self, position: InsertPosition, text: &str) {
        expect!(
            self.inner.insert_adjacent_text(position.as_web_sys(), text),
            "error calling Element::insert_adjacent_text - this is a bug!"
        )
    }
    fn matches(&self, selector: &str) -> Result<bool, InvalidSelector> {
        self.inner.matches(selector).map_err(|_| InvalidSelector)
    }
    fn query_selector(&self, selectors: &str) -> Result<Option<Element>, InvalidSelector> {
        self.inner
            .query_selector(selectors)
            .map(|opt| opt.map(Element::from_web_sys))
            .map_err(|_| InvalidSelector)
    }
    fn query_selector_all(&self, selectors: &str) -> Result<node::NodeList, InvalidSelector> {
        self.inner
            .query_selector_all(selectors)
            .map(node::NodeList::from_web_sys)
            .map_err(|_| InvalidSelector)
    }
    fn release_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId> {
        self.inner
            .release_pointer_capture(pointer_id)
            .map_err(|_| InvalidPointerId)
    }
    fn remove_attribute(&self, name: &str) {
        expect!(
            self.inner.remove_attribute(name),
            "error calling Element::remove_attribute - this is a bug!"
        )
    }
    fn remove_attribute_ns(&self, namespace: Option<&str>, local_name: &str) {
        expect!(
            self.inner.remove_attribute_ns(namespace, local_name),
            "error calling Element::remove_attribute_ns - this is a bug!"
        )
    }
    fn request_fullscreen(&self) {
        expect!(
            self.inner.request_fullscreen(),
            "error calling Element::request_fullscreen - this is a bug!"
        )
    }
    fn set_attribute(&self, name: &str, value: &str) -> Result<(), InvalidCharacter> {
        self.inner
            .set_attribute(name, value)
            .map_err(|_| InvalidCharacter)
    }
    fn set_attribute_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
        value: &str,
    ) -> Result<(), InvalidCharacter> {
        self.inner
            .set_attribute_ns(namespace, local_name, value)
            .map_err(|_| InvalidCharacter)
    }
    fn set_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId> {
        self.inner
            .set_pointer_capture(pointer_id)
            .map_err(|_| InvalidPointerId)
    }
    fn toggle_attribute(&self, name: &str) -> Result<bool, InvalidCharacter> {
        self.inner
            .toggle_attribute(name)
            .map_err(|_| InvalidCharacter)
    }
    fn namespace_uri(&self) -> Option<String> {
        self.inner.namespace_uri()
    }
    fn prefix(&self) -> Option<String> {
        self.inner.prefix()
    }
    fn local_name(&self) -> String {
        self.inner.local_name()
    }
    fn tag_name(&self) -> String {
        self.inner.tag_name()
    }
    fn id(&self) -> String {
        self.inner.id()
    }
    fn set_id(&self, id: &str) {
        self.inner.set_id(id)
    }
    fn class_name(&self) -> String {
        self.inner.class_name()
    }
    fn set_class_name(&self, class_name: &str) {
        self.inner.set_class_name(class_name)
    }
    //fn attributes(&self) -> NamedNodeMap {
    //unimplemented!()
    //}
    fn scroll_top(&self) -> i32 {
        self.inner.scroll_top()
    }
    fn set_scroll_top(&self, scroll_top: i32) {
        self.inner.set_scroll_top(scroll_top)
    }
    fn scroll_left(&self) -> i32 {
        self.inner.scroll_left()
    }
    fn set_scroll_left(&self, scroll_left: i32) {
        self.inner.set_scroll_left(scroll_left)
    }
    fn scroll_width(&self) -> i32 {
        self.inner.scroll_width()
    }
    fn scroll_height(&self) -> i32 {
        self.inner.scroll_height()
    }
    fn client_top(&self) -> i32 {
        self.inner.client_top()
    }
    fn client_left(&self) -> i32 {
        self.inner.client_left()
    }
    fn client_width(&self) -> i32 {
        self.inner.client_width()
    }
    fn client_height(&self) -> i32 {
        self.inner.client_height()
    }
    fn inner_html(&self) -> String {
        self.inner.inner_html()
    }
    fn set_inner_html(&self, inner_html: &str) {
        self.inner.set_inner_html(inner_html)
    }
    fn outer_html(&self) -> String {
        self.inner.outer_html()
    }
    fn set_outer_html(&self, outer_html: &str) {
        self.inner.set_outer_html(outer_html)
    }
    //fn children(&self) -> HtmlCollection {
    //    unimplemented!()
    //}
    fn first_element_child(&self) -> Option<Element> {
        self.inner.first_element_child().map(Element::from_web_sys)
    }
    fn last_element_child(&self) -> Option<Element> {
        self.inner.last_element_child().map(Element::from_web_sys)
    }
    fn child_element_count(&self) -> u32 {
        self.inner.child_element_count()
    }
}
