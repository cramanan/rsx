use std::collections::HashMap;

#[derive(Debug)]
pub enum Element {
    Element(HTMLElement),
    Text(String),
}

#[derive(Debug)]
pub struct HTMLElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
}
