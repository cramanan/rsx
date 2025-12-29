use std::{error::Error, fs::OpenOptions, io::Write};

use rsx::RSX;
use rsx_macro::rsx;

fn main() -> Result<(), Box<dyn Error>> {
    let name = "Alice";
    let element = rsx! (
        <html>
            <head><title>Title</title></head>
            <body>
                <h2>Hello {name}</h2>
            </body>
        </html>
    );
    println!("{:#?}", element);
    println!("{}", element.render());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("index.html")?;
    write!(file, "{}", element.render())?;
    Ok(())
}
