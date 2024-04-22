#[cfg(feature = "const_format")]
pub use const_format;
#[cfg(feature = "const_format")]
pub use fhtml_macros::formatcp;
pub use fhtml_macros::write;

#[macro_export]
macro_rules! format {
    ($($t:tt)*) => {{
        use ::std::fmt::Write;

        let mut output = String::new();
        let _ = $crate::write!(output, $($t)*);
        output
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_html() {
        assert_eq!(self::format!(<div></div>), "<div></div>");
        assert_eq!(
            self::format!(<p>"Hello, World!"</p>),
            "<p>Hello, World!</p>"
        );
    }

    #[test]
    fn test_nested_html() {
        assert_eq!(
            self::format!(<div><span>"Nested"</span>" content"</div>),
            "<div><span>Nested</span> content</div>"
        );
    }

    #[test]
    fn test_html_with_attributes() {
        assert_eq!(
            self::format!(<a href="https://example.com">"Link"</a>),
            r#"<a href="https://example.com">Link</a>"#
        );
        assert_eq!(
            self::format!(<span aria-hidden="true">"Hidden"</span>),
            r#"<span aria-hidden="true">Hidden</span>"#
        );
        assert_eq!(
            self::format!(<input type="email"/>),
            r#"<input type="email"/>"#
        );
    }

    #[test]
    fn test_dynamic_expressions() {
        let user = "Alice";
        assert_eq!(self::format!(<div>{user}</div>), "<div>Alice</div>");
        assert_eq!(self::format!(<div>{1 + 1}</div>), "<div>2</div>");
        assert_eq!(
            self::format!(<div class={std::format!("foo {}", "bar")}></div>),
            r#"<div class="foo bar"></div>"#
        );
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(
            self::format!(<div>"&lt;Encoded&gt;"</div>),
            "<div>&lt;Encoded&gt;</div>"
        );
    }
}
