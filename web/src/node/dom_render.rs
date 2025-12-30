use rsx_reactive::root::create_root;
use web_sys::wasm_bindgen::{JsCast, intern, prelude::Closure};

use crate::{document, node::dom_node::DomNode};

pub fn render_to<C: Fn() -> rsx::Element>(component: C, parent: &web_sys::Node) {
    // Do not call the destructor function, effectively leaking the scope.
    let _ = create_root(|| render_in_scope(component, parent));
}

impl From<rsx::Element> for DomNode {
    fn from(value: rsx::Element) -> Self {
        match value {
            rsx::Element::HTMLElement(element) => {
                let el = document().create_element(intern(&element.name)).unwrap();

                // Attributes
                for (name, value) in element.attributes {
                    el.set_attribute(&name, &value).unwrap();
                }

                for (name, listener) in element.event_listeners {
                    let closure = Closure::wrap(listener);
                    el.add_event_listener_with_callback(&name, closure.as_ref().unchecked_ref())
                        .unwrap();
                    closure.forget();
                }

                // Children (recursive!)
                for child in element.children {
                    let child_node = DomNode::from(child);
                    el.append_child(&child_node.raw).unwrap();
                }

                Self { raw: el.into() }
            }

            rsx::Element::Text(text) => Self {
                raw: document().create_text_node(&text).into(),
            },
        }
    }
}

pub fn render_in_scope<C: Fn() -> rsx::Element>(component: C, parent: &web_sys::Node) {
    let root = DomNode::from(component());
    parent.append_child(&root.raw).unwrap();
}
