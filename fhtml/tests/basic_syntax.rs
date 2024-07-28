#[test]
fn simple_tags() {
    assert_eq!(fhtml::format!(<foo></foo>).0, "<foo></foo>");
    assert_eq!(fhtml::format!(<bar></bar>).0, "<bar></bar>");
    assert_eq!(
        fhtml::format!(<foo></foo><bar></bar>).0,
        "<foo></foo><bar></bar>"
    );
}

#[test]
fn nested_tags() {
    assert_eq!(
        fhtml::format!(<foo><bar></bar></foo>).0,
        "<foo><bar></bar></foo>"
    );
    assert_eq!(
        fhtml::format!(<foo><bar><baz></baz></bar><qux></qux></foo>).0,
        "<foo><bar><baz></baz></bar><qux></qux></foo>"
    );
    assert_eq!(
        fhtml::format!(<foo><foo><foo></foo><baz></baz></foo></foo>).0,
        "<foo><foo><foo></foo><baz></baz></foo></foo>"
    );
}

#[test]
fn void_tags() {
    assert_eq!(fhtml::format!(<foo />).0, "<foo>");
    assert_eq!(fhtml::format!(<bar />).0, "<bar>");
    assert_eq!(fhtml::format!(<foo /><bar />).0, "<foo><bar>");
}

#[test]
fn simple_attributes() {
    assert_eq!(
        fhtml::format!(<foo bar="baz"></foo>).0,
        "<foo bar=\"baz\"></foo>"
    );
    assert_eq!(
        fhtml::format!(<foo bar="baz" qux="quux"></foo>).0,
        "<foo bar=\"baz\" qux=\"quux\"></foo>"
    );
    assert_eq!(
        fhtml::format!(<foo bar="baz" qux="quux"></foo><foo bar="baz"></foo>).0,
        "<foo bar=\"baz\" qux=\"quux\"></foo><foo bar=\"baz\"></foo>"
    );

    fhtml::format!(<div class={concat!("hello ", "world!")}></div>);
}
