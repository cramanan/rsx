use rsx::{RSX, rsx};

#[test]
fn simple_tag() {
    let element = rsx!(<button></button>);
    assert_eq!(element.name, "button")
}

#[test]
fn self_closing_tag() {
    let element = rsx!(<button />);
    assert_eq!(element.name, "button")
}

#[test]
fn simple_attributes() {
    let element = rsx!(<button id="button"></button>);
    assert_eq!(element.name, "button");
    assert_eq!(element.attributes.get("id"), Some(&"button".to_owned()))
}

#[test]
fn html_attributes() {
    let element = rsx!(<button id="button" disabled></button>);
    assert_eq!(element.name, "button");
    assert_eq!(element.attributes.get("id"), Some(&"button".to_owned()));
    assert_eq!(element.attributes.get("disabled"), Some(&true.to_string()))
}

#[test]
fn children() {
    let element = rsx!(
        <div>
            <p></p>
        </div>
    );

    assert_eq!(element.children.len(), 1);
    assert_eq!(element.children.first().unwrap().render(), "<p></p>")
}

#[test]
fn text_children() {
    let element = rsx!(<div>Hello</div>);

    assert_eq!(element.children.len(), 1);
    assert_eq!(element.children.first().unwrap().render(), "Hello")
}

#[test]
fn expression_children() {
    let variable = String::from("Hello");
    let element = rsx!(<div>{variable}</div>);

    assert_eq!(element.children.len(), 1);
    assert_eq!(
        element.children.first().unwrap().render(),
        variable.render()
    )
}

#[test]
fn mixed_children() {
    let variable = String::from("Alice");
    let element = rsx!(
        <div>
            <span></span>
            Hello {variable}
        </div>
    );

    assert_eq!(element.children.len(), 3);
    assert_eq!(element.children.first().unwrap().render(), "<span></span>");
    assert_eq!(element.children.get(1).unwrap().render(), "Hello");
    assert_eq!(element.children.last().unwrap().render(), variable.render())
}
