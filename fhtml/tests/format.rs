#[test]
fn doctype() {
    assert_eq!(fhtml::format!(<!DOCTYPE html>), "<!DOCTYPE html>");
    assert_eq!(fhtml::format!(<!doctype html>), "<!DOCTYPE html>");
}

#[test]
fn empty_tags() {
    assert_eq!(fhtml::format!(<div></div>), "<div></div>");
    assert_eq!(fhtml::format!(<span></span>), "<span></span>");
    assert_eq!(fhtml::format!(<b></b>), "<b></b>");
    assert_eq!(fhtml::format!(<body></body>), "<body></body>");
    assert_eq!(fhtml::format!(<br>), "<br>");
}

#[test]
fn nested_tags() {
    assert_eq!(
        fhtml::format!(<div><span></span></div>),
        "<div><span></span></div>"
    );
    assert_eq!(
        fhtml::format!(<div><footer></footer></div>),
        "<div><footer></footer></div>"
    );
    assert_eq!(
        fhtml::format!(<div><div><div><div></div></div></div></div>),
        "<div><div><div><div></div></div></div></div>"
    );
}

#[test]
fn tags_with_attributes() {
    assert_eq!(
        fhtml::format!(<div id="foo"></div>),
        "<div id=\"foo\"></div>"
    );
    assert_eq!(
        fhtml::format!(<p id="foo" class="bar"></p>),
        "<p id=\"foo\" class=\"bar\"></p>"
    );
    assert_eq!(
        fhtml::format!(<a href="https://docs.rs/fhtml"></a>),
        "<a href=\"https://docs.rs/fhtml\"></a>"
    );
}

#[test]
fn tags_with_dashes() {
    assert_eq!(fhtml::format!(<foo-bar></foo-bar>), "<foo-bar></foo-bar>");
    assert_eq!(
        fhtml::format!(<button hx-post="/foo/bar" hx-target="#bar"></button>),
        "<button hx-post=\"/foo/bar\" hx-target=\"#bar\"></button>"
    );
}

#[test]
fn self_closing_tags() {
    assert_eq!(fhtml::format!(<img />), "<img />");
    assert_eq!(fhtml::format!(<input />), "<input />");
    assert_eq!(fhtml::format!(<img />), "<img />");
}

#[test]
fn self_closing_tags_with_attributes() {
    assert_eq!(fhtml::format!(<br class="foo" />), "<br class=\"foo\" />");
    assert_eq!(
        fhtml::format!(<input class="foo bar" />),
        "<input class=\"foo bar\" />"
    );
    assert_eq!(
        fhtml::format!(<img src="/foo.webp" id="foo" class="foo bar" />),
        "<img src=\"/foo.webp\" id=\"foo\" class=\"foo bar\" />"
    );
}

#[test]
fn iterators() {
    assert_eq!(
        fhtml::format! {
            <ul>
                {
                    (0..10).map(|i| fhtml::format!(
                        <li>{i}</li>
                    )).collect::<Vec<_>>().join("")
                }
            </ul>
        },
        "<ul><li>0</li><li>1</li><li>2</li><li>3</li><li>4</li><li>5</\
         li><li>6</li><li>7</li><li>8</li><li>9</li></ul>"
    );
}

#[test]
fn expressions_escape() {
    assert_eq!(
        fhtml::format!(<div>{"<b>foo</b>":!}</div>),
        "<div>&lt;b&gt;foo&lt;/b&gt;</div>"
    );
    assert_eq!(fhtml::format!(<div>{"":!}</div>), "<div></div>");
    assert_eq!(
        fhtml::format!(<div>{String::from("<<>>"):!}</div>),
        "<div>&lt;&lt;&gt;&gt;</div>"
    );
}

#[test]
fn expressions_display() {
    assert_eq!(fhtml::format!(<div>{1 + 1}</div>), "<div>2</div>");
    assert_eq!(
        fhtml::format!(<div class={format!("foo {}", "bar")}></div>),
        "<div class=\"foo bar\"></div>"
    );
    let foo = "bar";
    assert_eq!(fhtml::format!(<div>{foo}</div>), "<div>bar</div>");
    assert_eq!(
        fhtml::format!(<html>{fhtml::format!(<head></head><body></body>)}</html>),
        "<html><head></head><body></body></html>",
    );
}

#[test]
fn expressions_debug() {
    assert_eq!(
        fhtml::format!(<code>{vec![1, 2, 3]:?}</code>),
        "<code>[1, 2, 3]</code>"
    );
    assert_eq!(fhtml::format!(<code>{1..10:?}</code>), "<code>1..10</code>");
    assert_eq!(
        fhtml::format!(<code>{Some("foo"):?}</code>),
        "<code>Some(\"foo\")</code>"
    );
}

#[test]
fn expressions_debug_alternate() {
    assert_eq!(
        fhtml::format!(<code>{vec![1, 2, 3]:#?}</code>),
        "<code>[\n    1,\n    2,\n    3,\n]</code>"
    );
    assert_eq!(
        fhtml::format!(<code>{1..10:#?}</code>),
        "<code>1..10</code>"
    );
    assert_eq!(
        fhtml::format!(<code>{Some("foo"):#?}</code>),
        "<code>Some(\n    \"foo\",\n)</code>"
    );
}

#[test]
fn expressions_binary() {
    assert_eq!(fhtml::format!(<div>{10:b}</div>), "<div>1010</div>");
}

#[test]
fn expressions_binary_alternate() {
    assert_eq!(fhtml::format!(<div>{10:#b}</div>), "<div>0b1010</div>");
}
