use std::borrow::Cow;

use crate::view::ViewNode;

mod dom_node;
pub mod dom_render;
pub use dom_render::render_to;

/// A trait that should be implemented for anything that represents an HTML node.
pub trait ViewHtmlNode: ViewNode {
    /// Create a new HTML element.
    fn create_element(tag: Cow<'static, str>) -> Self;
    /// Create a new HTML element with a XML namespace.
    fn create_element_ns(namespace: &'static str, tag: Cow<'static, str>) -> Self;
    /// Create a new HTML text node.
    fn create_text_node(text: Cow<'static, str>) -> Self;
    /// Create a new HTML text node whose value will be changed dynamically.
    fn create_dynamic_text_node(text: Cow<'static, str>) -> Self {
        Self::create_text_node(text)
    }
    /// Create a new HTML marker (comment) node.
    fn create_marker_node() -> Self;

    /// Set an HTML attribute.
    // fn set_attribute(&mut self, name: Cow<'static, str>, value: StringAttribute);
    /// Set a boolean HTML attribute.
    // fn set_bool_attribute(&mut self, name: Cow<'static, str>, value: BoolAttribute);
    /// Set a JS property on an element.
    // fn set_property(&mut self, name: Cow<'static, str>, value: MaybeDyn<JsValue>);
    /// Set an event handler on an element.
    fn set_event_handler(
        &mut self,
        name: Cow<'static, str>,
        handler: impl FnMut(web_sys::Event) + 'static,
    );
    /// Set the inner HTML value of an element.
    fn set_inner_html(&mut self, inner_html: Cow<'static, str>);

    /// Return the raw web-sys node.
    fn as_web_sys(&self) -> &web_sys::Node;
    /// Wrap a raw web-sys node.
    fn from_web_sys(node: web_sys::Node) -> Self;
}
