#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "const-format")))]
#[cfg(feature = "const-format")]
pub use fhtml_macros::formatcp;
pub use fhtml_macros::write;

#[doc(hidden)]
pub mod _internal {
    #[cfg(feature = "const-format")]
    pub use ::const_format::*;
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
/// fhtml::format!(<div>"Hello, World!"</div>); // "<div>Hello, World!</div>"
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
/// let escaped = escape(raw);
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
