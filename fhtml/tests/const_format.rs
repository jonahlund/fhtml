#![cfg(feature = "const")]

#[test]
fn valid_expressions() {
    assert_eq!(fhtml::formatcp!(<div>{10_u8}</div>), "<div>10</div>");
    assert_eq!(fhtml::formatcp!(<div>{"foo"}</div>), "<div>foo</div>");
}
