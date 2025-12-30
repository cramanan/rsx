pub mod node;
mod utils;
mod view;

/// Utility function for accessing the global [`web_sys::Window`] object.
pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

/// Utility function for accessing the global [`web_sys::Document`] object.
pub fn document() -> web_sys::Document {
    thread_local! {
        /// Cache for small performance improvement by preventing repeated calls to `window().document()`.
        static DOCUMENT: web_sys::Document = window().document().expect("no `document` exists");
    }
    DOCUMENT.with(Clone::clone)
}
