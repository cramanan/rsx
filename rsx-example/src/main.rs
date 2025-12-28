use rsx_macro::rsx;

fn main() {
    let element = rsx! (
        <div name="value">
            <input value="Hello World"></input>
        </div>
    );
    println!("{:#?}", element);
}
