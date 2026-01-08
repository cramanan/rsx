pub mod component;

pub use rsx_macros::*;

use std::collections::HashMap;

pub enum Element {
    HTMLElement(HTMLElement),
    Text(String),
}

impl<T: ToString> From<T> for Element {
    fn from(value: T) -> Self {
        Element::Text(value.to_string())
    }
}

impl From<HTMLElement> for Element {
    fn from(value: HTMLElement) -> Self {
        Element::HTMLElement(value)
    }
}

pub type EventListener = Box<dyn Fn(web_sys::Event)>;

pub struct HTMLElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub event_listeners: HashMap<String, EventListener>,
    pub children: Vec<Element>,
}
