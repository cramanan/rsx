use rsx::RSX;
use rsx_macro::rsx;

fn main() {
    let name = 0;
    let element = rsx! (
        <div class="container">
            <h2>Hello {name} {name}</h2>
        </div>
    );
    println!("{:#?}", element);
    println!("{}", element.render());
}
