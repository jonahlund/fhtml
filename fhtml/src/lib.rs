//! `fhtml` - Formatting macros tailored for HTML.
//!
//! This crate provides macros for building HTML content, enabling the direct
//! integration of dynamic expressions and standard formatting into HTML tags.
//! This approach simplifies the creation of HTML, improving both its
//! readability and ease of use.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
pub mod _internal {
    #[cfg(feature = "const_format")]
    pub use ::const_format::*;
}

/// Writes HTML content with embedded expressions into a buffer.
///
/// This macro uses `std::write!` to format HTML content, allowing the
/// inclusion of expressions within curly braces `{}`.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// use std::fmt::Write;
///
/// let mut buffer = String::new();
/// fhtml::write!(buffer, <div>{1 + 1}</div>);
///
/// assert_eq!(buffer, "<div>2</div>");
/// ```
pub use fhtml_macros::write;

/// Generates a `String` with formatted HTML content.
///
/// This macro functions similarly to `std::format!`, but is tailored for HTML
/// content, allowing direct embedding of expressions within curly braces `{}`.
///
/// ## Examples
///
/// Simple HTML generation:
///
/// ```
/// let output = fhtml::format! {
///     <div>{10 + 20}</div>
/// };
///
/// assert_eq!(output, "<div>30</div>");
/// ```
#[macro_export]
macro_rules! format {
    ($($t:tt)*) => {{
        use ::std::fmt::Write;

        let mut output = String::new();
        let _ = $crate::write!(output, $($t)*);
        output
    }};
}

/// Generates a compile-time constant `&'static str` with formatted HTML.
///
/// This macro is enabled with the `const_format` feature and allows
/// embedding of expressions within HTML content at compile time. Note that
/// this feature imposes certain limitations on the expressions that can be
/// used.
///
/// ## Examples
///
/// Generating constant HTML:
///
/// ```
/// const HTML: &'static str = fhtml::formatcp! {
///     <div>{10_u8 + 20_u8}</div>
/// };
///
/// assert_eq!(HTML, "<div>30</div>");
/// ```
///
/// More information can be found on [docs.rs](https://docs.rs/const_format).
#[cfg_attr(docsrs, doc(cfg(feature = "const_format")))]
#[cfg(feature = "const_format")]
pub use fhtml_macros::formatcp;

/// Escapes special HTML characters in a string
///
/// This function converts:
///
/// - `&` to `&amp;`
/// - `<` to `&lt;`
/// - `>` to `&gt;`
/// - `"` to `&quot;`
/// - `'` to `&#39;`
///
/// # Arguments
///
/// * `input` - A string slice that may contain special HTML characters.
///
/// # Returns
///
/// A `String` with all special HTML characters replaced by their respective
/// HTML entities.
///
/// # Examples
///
/// ```
/// let raw_html = "5 < 7 & 5 > 3";
/// let safe_html = fhtml::escape(raw_html);
/// assert_eq!(safe_html, "5 &lt; 7 &amp; 5 &gt; 3");
/// ```
#[inline]
pub fn escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_html() {
        assert_eq!(self::format!(<div></div>), "<div></div>");
        assert_eq!(
            self::format!(<p>"Hello, World!"</p>),
            "<p>Hello, World!</p>"
        );
    }

    #[test]
    fn nested_html() {
        assert_eq!(
            self::format!(<div><span>"Nested"</span>" content"</div>),
            "<div><span>Nested</span> content</div>"
        );
    }

    #[test]
    fn html_with_attributes() {
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
    fn dynamic_expressions() {
        let user = "Alice";
        assert_eq!(self::format!(<div>{user}</div>), "<div>Alice</div>");
        assert_eq!(self::format!(<div>{1 + 1}</div>), "<div>2</div>");
        assert_eq!(
            self::format!(<div class={std::format!("foo {}", "bar")}></div>),
            r#"<div class="foo bar"></div>"#
        );
    }

    #[test]
    fn formatting_specifiers() {
        assert_eq!(self::format!(<div>{0..10:?}</div>), "<div>0..10</div>");
        assert_eq!(
            self::format!(<div>{vec![0, 1, 2]:?}</div>),
            "<div>[0, 1, 2]</div>"
        );
        assert_eq!(self::format!(<div>{10:x?}</div>), "<div>a</div>");
        assert_eq!(self::format!(<div>{10:X?}</div>), "<div>A</div>");
        assert_eq!(self::format!(<div>{10:o}</div>), "<div>12</div>");
        assert_eq!(self::format!(<div>{10:x}</div>), "<div>a</div>");
        assert_eq!(self::format!(<div>{10:X}</div>), "<div>A</div>");
        assert_eq!(self::format!(<div>{10:b}</div>), "<div>1010</div>");
        assert_eq!(self::format!(<div>{10:e}</div>), "<div>1e1</div>");
        assert_eq!(self::format!(<div>{10:E}</div>), "<div>1E1</div>");
    }

    #[test]
    fn escape_mixed() {
        let text = "Hello & Goodbye <script>alert('Hi');</script>";
        let expected = "Hello &amp; Goodbye \
                        &lt;script&gt;alert(&#39;Hi&#39;);&lt;/script&gt;";
        assert_eq!(escape(text), expected);
    }
}
