use rsx_macro::rsx;

fn main() {
    let name = 0;
    let element = rsx! (
        <h2 name="value">Hello {name} {name}</h2>
    );
    println!("{:#?}", element);
}
