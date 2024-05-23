#![cfg(feature = "const")]

#[test]
fn valid_expressions() {
    assert_eq!(fhtml::const_format!(<div>{10_u8}</div>), "<div>10</div>");
    assert_eq!(fhtml::const_format!(<div>{"foo"}</div>), "<div>foo</div>");
}
