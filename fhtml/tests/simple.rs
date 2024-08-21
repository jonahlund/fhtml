#[test]
fn it_works() {
    let abc = String::new();
    fhtml::format_args!(<div>{abc}</div>);
}
