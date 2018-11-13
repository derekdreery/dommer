use wasm_bindgen::prelude::*;

use crate::{element, event};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NodeType {
    Element,
    Attribute,
    Text,
    CdataSection,
    EntityReference,
    EntityNode,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
    Notation,
}

impl NodeType {
    pub(crate) fn from_web_sys(raw: u16) -> NodeType {
        match raw {
            web_sys::Node::ELEMENT_NODE => NodeType::Element,
            web_sys::Node::ATTRIBUTE_NODE => NodeType::Attribute,
            web_sys::Node::TEXT_NODE => NodeType::Text,
            web_sys::Node::CDATA_SECTION_NODE => NodeType::CdataSection,
            web_sys::Node::ENTITY_REFERENCE_NODE => NodeType::EntityReference,
            web_sys::Node::ENTITY_NODE => NodeType::EntityNode,
            web_sys::Node::PROCESSING_INSTRUCTION_NODE => NodeType::ProcessingInstruction,
            web_sys::Node::COMMENT_NODE => NodeType::Comment,
            web_sys::Node::DOCUMENT_NODE => NodeType::Document,
            web_sys::Node::DOCUMENT_TYPE_NODE => NodeType::DocumentType,
            web_sys::Node::DOCUMENT_FRAGMENT_NODE => NodeType::DocumentFragment,
            web_sys::Node::NOTATION_NODE => NodeType::Notation,
            _ => unreachable!("invalid node type"),
        }
    }
}

bitflags::bitflags! {
    pub struct DocumentPosition: u16 {
        const DISCONNECTED = web_sys::Node::DOCUMENT_POSITION_DISCONNECTED;
        const PRECEDING = web_sys::Node::DOCUMENT_POSITION_PRECEDING;
        const FOLLOWING = web_sys::Node::DOCUMENT_POSITION_FOLLOWING;
        const CONTAINS = web_sys::Node::DOCUMENT_POSITION_CONTAINS;
        const CONTAINED_BY = web_sys::Node::DOCUMENT_POSITION_CONTAINED_BY;
        const IMPLEMENTATION_SPECIFIC = web_sys::Node::DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC;
    }
}

// todo explore using dynamic dispatch - profile size/speed and check ease of use.
pub trait INode: event::IEventTarget {
    // I believe this cannot error because we check the type of the parameter at compile-time.
    fn append_child(&self, node: impl AsRef<Node>);
    // I believe this cannot error
    fn clone_node(&self, deep: bool) -> Node;
    fn compare_document_position(&self, other: impl Into<Node>) -> DocumentPosition;
    fn contains(&self, other: impl Into<Node>) -> bool;
    fn get_root_node(&self) -> Node;
    fn has_child_nodes(&self) -> bool;
    fn insert_before(&self, node: impl Into<Node>, child: Option<impl Into<Node>>);
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool;
    // todo can self ever be null - does node need to be optional?
    fn is_equal_node(&self, node: impl Into<Node>) -> bool;
    // todo can self ever be null - does node need to be optional?
    fn is_same_node(&self, node: impl Into<Node>) -> bool;
    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<String>;
    fn lookup_prefix(&self, namespace: Option<&str>) -> Option<String>;
    fn normalize(&self);
    // todo type error
    // The method throws an exception in 2 different ways:
    //
    // If the child was in fact a child of element and so existing on the DOM,
    // but was removed the method throws the following exception:
    // `Uncaught NotFoundError: Failed to execute 'removeChild' on 'Node':
    // The node to be removed is not a child of this node.`
    //
    // If the child doesn't exist on the DOM of the page,
    // the method throws the following exception:
    // `Uncaught TypeError: Failed to execute 'removeChild' on 'Node':
    // parameter 1 is not of type 'Node'.`
    fn remove_child(&self, child: impl Into<Node>) -> Result<Node, JsValue>;
    // todo type error & do we need to return parameter?
    fn replace_child(&self, node: impl Into<Node>, child: impl Into<Node>)
        -> Result<Node, JsValue>;
    fn node_type(&self) -> NodeType;
    fn node_name(&self) -> String;
    // todo type error
    fn base_uri(&self) -> Result<Option<String>, JsValue>;
    fn is_connected(&self) -> bool;
    //fn owner_document(&self) -> Option<Document>;
    fn parent_node(&self) -> Option<Node>;
    fn parent_element(&self) -> Option<element::Element>;
    fn first_child(&self) -> Option<Node>;
    fn last_child(&self) -> Option<Node>;
    fn previous_sibling(&self) -> Option<Node>;
    fn next_sibling(&self) -> Option<Node>;
    fn node_value(&self) -> Option<String>;
    fn set_node_value(&self, node_value: Option<&str>);
    fn text_content(&self) -> Option<String>;
    fn set_text_content(&self, text_content: Option<&str>);
}

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) inner: web_sys::Node,
}

impl AsRef<web_sys::EventTarget> for Node {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl From<Node> for event::EventTarget {
    fn from(node: Node) -> event::EventTarget {
        event::EventTarget {
            inner: node.inner.into(),
        }
    }
}

impl Node {
    /// Wrap a web_sys::Node
    pub(crate) fn from_web_sys(inner: web_sys::Node) -> Node {
        Node { inner }
    }
}

impl event::IEventTarget for Node {
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

impl INode for Node {
    fn append_child(&self, node: impl AsRef<Node>) {
        expect!(
            self.inner.append_child(&node.as_ref().inner),
            "unexpected failure of Node::append_child - this is a bug!"
        );
    }
    fn clone_node(&self, deep: bool) -> Node {
        let node = expect!(
            self.inner.clone_node_with_deep(deep),
            "unexpected failure of Node::clone_node - this is a bug!"
        );
        Node::from_web_sys(node)
    }
    fn compare_document_position(&self, other: impl Into<Node>) -> DocumentPosition {
        DocumentPosition::from_bits_truncate(
            self.inner.compare_document_position(&other.into().inner),
        )
    }
    fn contains(&self, other: impl Into<Node>) -> bool {
        self.inner.contains(Some(&other.into().inner))
    }
    fn get_root_node(&self) -> Node {
        Node::from_web_sys(self.inner.get_root_node())
    }
    fn has_child_nodes(&self) -> bool {
        self.inner.has_child_nodes()
    }
    fn insert_before(&self, node: impl Into<Node>, child: Option<impl Into<Node>>) {
        expect!(
            self.inner.insert_before(
                &node.into().inner,
                child.map(Into::into).as_ref().map(|val| &val.inner),
            ),
            "unexpected failure of Node::insert_before - this is a bug!"
        );
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        self.inner.is_default_namespace(namespace)
    }
    fn is_equal_node(&self, node: impl Into<Node>) -> bool {
        self.inner.is_equal_node(Some(&node.into().inner))
    }
    fn is_same_node(&self, node: impl Into<Node>) -> bool {
        self.inner.is_same_node(Some(&node.into().inner))
    }
    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<String> {
        self.inner.lookup_namespace_uri(prefix)
    }
    fn lookup_prefix(&self, namespace: Option<&str>) -> Option<String> {
        self.inner.lookup_prefix(namespace)
    }
    fn normalize(&self) {
        self.inner.normalize()
    }
    fn remove_child(&self, child: impl Into<Node>) -> Result<Node, JsValue> {
        self.inner.remove_child(&child.into().inner).map(Node::from_web_sys)
    }
    fn replace_child(
        &self,
        node: impl Into<Node>,
        child: impl Into<Node>,
    ) -> Result<Node, JsValue> {
        self.inner
            .replace_child(&node.into().inner, &child.into().inner)
            .map(Node::from_web_sys)
    }
    fn node_type(&self) -> NodeType {
        NodeType::from_web_sys(self.inner.node_type())
    }
    fn node_name(&self) -> String {
        self.inner.node_name()
    }
    fn base_uri(&self) -> Result<Option<String>, JsValue> {
        self.inner.base_uri()
    }
    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }
    //fn owner_document(&self) -> Option<Document> {
    //unimplemented!()

    //}
    fn parent_node(&self) -> Option<Node> {
        self.inner.parent_node().map(Node::from_web_sys)
    }
    fn parent_element(&self) -> Option<element::Element> {
        self.inner
            .parent_element()
            .map(element::Element::from_web_sys)
    }
    fn first_child(&self) -> Option<Node> {
        self.inner.first_child().map(Node::from_web_sys)
    }
    fn last_child(&self) -> Option<Node> {
        self.inner.last_child().map(Node::from_web_sys)
    }
    fn previous_sibling(&self) -> Option<Node> {
        self.inner.previous_sibling().map(Node::from_web_sys)
    }
    fn next_sibling(&self) -> Option<Node> {
        self.inner.next_sibling().map(Node::from_web_sys)
    }
    fn node_value(&self) -> Option<String> {
        self.inner.node_value()
    }
    fn set_node_value(&self, node_value: Option<&str>) {
        self.inner.set_node_value(node_value)
    }
    fn text_content(&self) -> Option<String> {
        self.inner.text_content()
    }
    fn set_text_content(&self, text_content: Option<&str>) {
        self.inner.set_text_content(text_content)
    }
}

pub struct NodeList {
    inner: web_sys::NodeList,
}

impl NodeList {
    pub(crate) fn from_web_sys(inner: web_sys::NodeList) -> Self {
        NodeList { inner }
    }
}

impl NodeList {
    /// The number of nodes in this collection
    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn get(&self, idx: usize) -> Option<Node> {
        self.inner.get(idx as u32).map(|inner| Node { inner })
    }
}

impl IntoIterator for NodeList {
    type Item = Node;
    type IntoIter = NodeListIterator;
    fn into_iter(self) -> Self::IntoIter {
        NodeListIterator {
            idx: 0,
            len: self.inner.length() as usize,
            inner: self.inner,
        }
    }
}

pub struct NodeListIterator {
    idx: usize,
    len: usize,
    inner: web_sys::NodeList,
}

impl Iterator for NodeListIterator {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None;
        }
        let inner = expect_opt!(
            self.inner.get(self.idx as u32),
            "exception indexing into NodeList - this is a bug!"
        );
        self.idx += 1;
        Some(Node { inner })
    }
}
