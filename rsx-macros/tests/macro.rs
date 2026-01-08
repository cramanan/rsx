use rsx::rsx;

#[test]
fn should_compile() {
    let _ = rsx!(<h1 prop="">Hello</h1>);
}
