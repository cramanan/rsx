use rsx::rsx;

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
}

#[test]
fn text_children() {
    let element = rsx!(<div>Hello</div>);

    assert_eq!(element.children.len(), 1);
}

#[test]
fn expression_children() {
    let variable = String::from("Hello");
    let element = rsx!(<div>{variable}</div>);

    assert_eq!(element.children.len(), 1);
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
}

#[test]
fn event_listeners() {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Shared state to track if the handler was called
    let called = Rc::new(RefCell::new(false));
    let called_clone = called.clone();

    let onclick = move || {
        *called_clone.borrow_mut() = true;
    };

    let element = rsx!(<button name="value" onclick={onclick}>Click</button>);

    // Simulate event
    if let Some(handler) = element.event_handlers.get("onclick") {
        handler(); // Call the handler
    }

    // Assert the handler ran
    assert!(*called.borrow());
}
