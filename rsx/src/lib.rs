pub use rsx_macros::*;

use std::collections::HashMap;

pub enum Node {
    HTMLElement(HTMLElement),
    Text(String),
}

pub trait RSX {
    fn as_node(&self) -> Node;
    fn as_element(&self) -> Element;
}

impl RSX for HTMLElement {
    fn as_node(&self) -> Node {
        Node::HTMLElement(self.clone())
    }

    fn as_element(&self) -> Element {
        Box::new(self.clone())
    }
}

pub type Element = Box<dyn RSX>;

impl Clone for Element {
    fn clone(&self) -> Self {
        self.as_element()
    }
}

impl RSX for String {
    fn as_node(&self) -> Node {
        Node::Text(self.to_owned())
    }

    fn as_element(&self) -> Element {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct HTMLElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
}
