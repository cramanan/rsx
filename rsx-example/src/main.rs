use rsx_macro::rsx;

fn component() -> rsx::Element {
    return rsx!(
        <h1></h1>
    );
}

fn main() {
    let element = rsx! (
        <h2 name="value">Hello World</h2>
    );
    println!("{:#?}", element);
}
