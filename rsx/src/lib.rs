pub use rsx_macros::*;

use std::{collections::HashMap, fmt::Debug, ops::Deref};

pub trait RSX {
    fn render(&self) -> String;
}

impl RSX for HTMLElement {
    fn render(&self) -> String {
        let children = self
            .children
            .iter()
            .map(RSX::render)
            .collect::<Vec<_>>()
            .join(" ");
        format!("<{}>{}</{}>", self.name, children, self.name)
    }
}

pub type Element = Box<dyn RSX>;

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

impl RSX for Element {
    fn render(&self) -> String {
        self.deref().render()
    }
}

impl RSX for String {
    fn render(&self) -> String {
        self.clone()
    }
}

#[derive(Debug)]
pub struct HTMLElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
}
