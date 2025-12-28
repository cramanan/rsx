use rsx_macro::rsx;

fn main() {
    let element = rsx! (
        <div name="value"></div>
    );
    println!("{:#?}", element);
}
