use rsx::rsx;
use rsx_web::{document, node::render_to};
use wasm_bindgen::prelude::*;
use web_sys::console;

fn app() -> rsx::Element {
    let onclick = |_| console::log_1(&"Hello from the WASM side !".into());
    return rsx!(
        <main>
            <h1>This is a RSX snippet</h1>
            <button onclick={onclick} name="button">Click me and look at the console</button>
        </main>
    );
}

#[wasm_bindgen(start)]
fn start() {
    // Panic hook that logs errors using JS console.error
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let root = document()
        .get_element_by_id("app")
        .expect("#app is undefined");

    render_to(app, &root);
}
