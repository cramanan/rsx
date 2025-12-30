use rsx::rsx;
// use rsx_reactive::signals::create_signal;
use rsx_web::{console_log, document, node::render_to};
use wasm_bindgen::prelude::*;

fn app() -> rsx::Element {
    // let count = create_signal(0);
    let onclick = move |_| {
        console_log!("Hello from the WASM side !");
        // console_log!("{}", count.get());
        // count.set(count.get() + 1);
    };

    return rsx!(
        <main>
            <h1>This is a RSX snippet</h1>
            <button onclick={onclick} name="button">Click me and look at the console</button>
            // <div>{count.get().to_string()}</div>
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
