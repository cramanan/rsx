use std::{collections::HashMap, fmt::Debug};

pub trait RSX {
    fn render(&self) -> String;
}

impl RSX for HTMLElement {
    fn render(&self) -> String {
        format!("<{}>...</{}>", self.name, self.name)
    }
}

impl RSX for String {
    fn render(&self) -> String {
        self.clone()
    }
}

pub struct HTMLElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Box<dyn RSX>>,
}

impl Debug for HTMLElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HTMLElement")
            .field("name", &self.name)
            .field("attributes", &self.attributes)
            .field(
                "children",
                &self.children.iter().map(|e| e.render()).collect::<Vec<_>>(),
            )
            .finish()
    }
}
