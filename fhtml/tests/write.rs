#[test]
fn write_to_io_buffer() {
    use std::io::Write;

    let mut buffer = vec![];
    fhtml::write!(buffer, <div>"foo"</div>).unwrap();

    assert_eq!(buffer, b"<div>foo</div>");
}

#[test]
fn write_to_string() {
    use std::fmt::Write;

    let mut buffer = String::new();
    fhtml::write!(buffer, <div>"foo"</div>).unwrap();

    assert_eq!(buffer, "<div>foo</div>");
}
