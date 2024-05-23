#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "const")))]
#[cfg(feature = "const")]
pub use fhtml_macros::const_format;
pub use fhtml_macros::format_args;

#[doc(hidden)]
pub mod _internal {
    #[cfg(feature = "const")]
    pub use ::const_format::*;
}

/// Writes formatted HTML to a buffer.
///
/// `fhtml::write!` works similar to [`std::write!`] with a few key differences:
/// - HTML can be written as-is without having to be inside a string literal.
/// - Expressions are written directly inside braces, compared to
///   [`std::write!`], where they are passed as separate parameters.
///
/// Formatting specifiers are written after expressions, denoted by
/// a colon `:`, similar to how they are written in [`std::write!`].
///
/// Values are not escaped implicitly, but are opt-in with an exclamation mark
/// `!` preceding any formatting specifiers:
/// `{[expr]:![specifiers]}`.
///
/// [`std::write!`]: std::write
///
/// # Examples
///
/// ## Simple usage
///
/// ```rust
/// use std::fmt::Write;
/// let mut buffer = String::new();
/// let _ = fhtml::write!(buffer, <div>"Hello, World!"</div>);
/// assert_eq!(buffer,  "<div>Hello, World!</div>");
/// ```
///
/// ## Escaping values
///
/// ```rust
/// use std::fmt::Write;
/// let mut buffer = String::new();
/// let _ = fhtml::write!(buffer, <div>{"<b>foo</b>":!}</div>);
/// assert_eq!(buffer, "<div>&lt;b&gt;foo&lt;/b&gt;</div>");
/// ```
#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {{
        $dst.write_fmt($crate::format_args!($($arg)*))
    }};
}

/// Writes formatted HTML to a `String`.
///
/// Similar to [`std::format!`], this macro returns an owned `String` with the
/// formatted content.
///
/// The syntax and overall behaviour is identical to [`fhtml::write!`], where
/// more detailed documentation can be found.
///
/// [`std::format!`]: std::format
/// [`fhtml::write!`]: crate::write
///
/// # Examples
///
/// ## Simple usage
///
/// ```rust
/// use std::fmt::Write;
/// fhtml::format!(<div>"Hello, World!"</div>); // "<div>Hello, World!</div>"
/// ```
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = ::std::fmt::format($crate::format_args!($($arg)*));
        res
    }};
}

/// Escapes special HTML characters in a string.
///
/// This function takes an input string and replaces certain characters with
/// their corresponding HTML escape sequences. The following characters are
/// escaped:
///
/// - `&` becomes `&amp;`
/// - `<` becomes `&lt;`
/// - `>` becomes `&gt;`
/// - `"` becomes `&quot;`
/// - `'` becomes `&#39;`
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let raw = "Hello, <world> & 'friends'";
/// let escaped = fhtml::escape(raw);
/// assert_eq!(escaped, "Hello, &lt;world&gt; &amp; &#39;friends&#39;");
/// ```
///
/// This is useful for preventing HTML injection attacks when displaying
/// untrusted input in a web page.
///
/// # Parameters
///
/// - `input`: A value that can be converted to a string slice (`&str`). This
///   includes `String` and `&str`.
///
/// # Returns
///
/// A `String` with the special HTML characters replaced by their escape
/// sequences.
///
/// # Performance
///
/// The function preallocates a `String` with the same length as the input,
/// which could save some reallocation costs if the input contains few
/// characters that need to be escaped.
#[inline]
pub fn escape<T: AsRef<str>>(input: T) -> String {
    let input = input.as_ref();
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
    fn escape_ampersand() {
        assert_eq!(escape("&"), "&amp;");
        assert_eq!(escape("This & that"), "This &amp; that");
    }

    #[test]
    fn escape_less_than() {
        assert_eq!(escape("<"), "&lt;");
        assert_eq!(escape("a < b"), "a &lt; b");
    }

    #[test]
    fn escape_greater_than() {
        assert_eq!(escape(">"), "&gt;");
        assert_eq!(escape("a > b"), "a &gt; b");
    }

    #[test]
    fn escape_double_quote() {
        assert_eq!(escape("\""), "&quot;");
        assert_eq!(escape("She said \"hello\""), "She said &quot;hello&quot;");
    }

    #[test]
    fn escape_single_quote() {
        assert_eq!(escape("'"), "&#39;");
        assert_eq!(escape("It's a test"), "It&#39;s a test");
    }

    #[test]
    fn escape_multiple_special_chars() {
        assert_eq!(
            escape("<div> & \"text\""),
            "&lt;div&gt; &amp; &quot;text&quot;"
        );
        assert_eq!(escape("5 > 3 & 2 < 4"), "5 &gt; 3 &amp; 2 &lt; 4");
    }

    #[test]
    fn escape_no_special_chars() {
        assert_eq!(escape("Hello, world!"), "Hello, world!");
        assert_eq!(escape("Rust programming"), "Rust programming");
    }

    #[test]
    fn escape_empty_string() {
        assert_eq!(escape(""), "");
    }

    #[test]
    fn escape_mixed_input() {
        assert_eq!(
            escape("The price is 5 > 4 & 3 < 6"),
            "The price is 5 &gt; 4 &amp; 3 &lt; 6"
        );
        assert_eq!(
            escape("Use 'single' and \"double\" quotes"),
            "Use &#39;single&#39; and &quot;double&quot; quotes"
        );
    }
}
