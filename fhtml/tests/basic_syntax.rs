use std::fmt::Write;
#[test]
fn it_works() {
    fhtml::format!(<div>{1 + 2}</div>);
    fhtml::write!(String::new(), <div>{1 + 2}</div>);
}
