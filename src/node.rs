use wasm_bindgen::prelude::*;

use crate::{/*element, */event};

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

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) inner: web_sys::Node,
}

impl std::ops::Deref for Node {
    type Target = event::EventTarget;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&web_sys::EventTarget, &event::EventTarget>(&*self.inner) }
    }
}

impl Node {
    /// Wrap a web_sys::Node
    pub(crate) fn from_web_sys(inner: web_sys::Node) -> Node {
        Node { inner }
    }

    // I believe this cannot error because we check the type of the parameter at compile-time.
    pub fn append_child(&self, node: &Node) {
        expect!(
            self.inner.append_child(&node.inner),
            "calling Node::append_child"
        );
    }
    // I believe this cannot error
    pub fn clone_node(&self, deep: bool) -> Node {
        let node = expect!(
            self.inner.clone_node_with_deep(deep),
            "calling Node::clone_node"
        );
        Node::from_web_sys(node)
    }
    pub fn compare_document_position(&self, other: impl Into<Node>) -> DocumentPosition {
        DocumentPosition::from_bits_truncate(
            self.inner.compare_document_position(&other.into().inner),
        )
    }
    pub fn contains(&self, other: impl Into<Node>) -> bool {
        self.inner.contains(Some(&other.into().inner))
    }
    pub fn get_root_node(&self) -> Node {
        Node::from_web_sys(self.inner.get_root_node())
    }
    pub fn has_child_nodes(&self) -> bool {
        self.inner.has_child_nodes()
    }
    pub fn insert_before(&self, node: impl Into<Node>, child: Option<impl Into<Node>>) {
        expect!(
            self.inner.insert_before(
                &node.into().inner,
                child.map(Into::into).as_ref().map(|val| &val.inner),
            ),
            "calling Node::insert_before"
        );
    }
    pub fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        self.inner.is_default_namespace(namespace)
    }
    // todo can self ever be null - does node need to be optional?
    pub fn is_equal_node(&self, node: impl Into<Node>) -> bool {
        self.inner.is_equal_node(Some(&node.into().inner))
    }
    // todo can self ever be null - does node need to be optional?
    pub fn is_same_node(&self, node: impl Into<Node>) -> bool {
        self.inner.is_same_node(Some(&node.into().inner))
    }
    pub fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<String> {
        self.inner.lookup_namespace_uri(prefix)
    }
    pub fn lookup_prefix(&self, namespace: Option<&str>) -> Option<String> {
        self.inner.lookup_prefix(namespace)
    }
    pub fn normalize(&self) {
        self.inner.normalize()
    }
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
    pub fn remove_child(&self, child: impl Into<Node>) -> Result<Node, JsValue> {
        self.inner.remove_child(&child.into().inner).map(Node::from_web_sys)
    }
    // todo type error & do we need to return parameter?
    pub fn replace_child(
        &self,
        node: impl Into<Node>,
        child: impl Into<Node>,
    ) -> Result<Node, JsValue> {
        self.inner
            .replace_child(&node.into().inner, &child.into().inner)
            .map(Node::from_web_sys)
    }
    pub fn node_type(&self) -> NodeType {
        NodeType::from_web_sys(self.inner.node_type())
    }
    pub fn node_name(&self) -> String {
        self.inner.node_name()
    }
    // todo type error
    pub fn base_uri(&self) -> Result<Option<String>, JsValue> {
        self.inner.base_uri()
    }
    pub fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }
    //pub fn owner_document(&self) -> Option<Document> {
    //unimplemented!()

    //}
    pub fn parent_node(&self) -> Option<Node> {
        self.inner.parent_node().map(Node::from_web_sys)
    }
    /*pub fn parent_element(&self) -> Option<element::Element> {
        self.inner
            .parent_element()
            .map(element::Element::from_web_sys)
    }*/
    pub fn first_child(&self) -> Option<Node> {
        self.inner.first_child().map(Node::from_web_sys)
    }
    pub fn last_child(&self) -> Option<Node> {
        self.inner.last_child().map(Node::from_web_sys)
    }
    pub fn previous_sibling(&self) -> Option<Node> {
        self.inner.previous_sibling().map(Node::from_web_sys)
    }
    pub fn next_sibling(&self) -> Option<Node> {
        self.inner.next_sibling().map(Node::from_web_sys)
    }
    pub fn node_value(&self) -> Option<String> {
        self.inner.node_value()
    }
    pub fn set_node_value(&self, node_value: Option<&str>) {
        self.inner.set_node_value(node_value)
    }
    pub fn text_content(&self) -> Option<String> {
        self.inner.text_content()
    }
    pub fn set_text_content(&self, text_content: Option<&str>) {
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
            "out of bounds error indexing into NodeList"
        );
        self.idx += 1;
        Some(Node { inner })
    }
}

impl std::iter::FusedIterator for NodeListIterator {}
