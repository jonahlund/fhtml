use crate::fmt::Display;

/// Escapes all special HTML characters in `input` and returns the result.
///
/// The following characters are escaped:
///
///   '&' -> `&amp;`
///   '<' -> `&lt;`
///   '>' -> `&gt;`
///   '"' -> `&quot;`
///
/// All other characters remain unchanged.
#[inline]
pub fn escape(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    escape_into(input, &mut buf);
    buf
}

/// Escapes all special HTML characters in `input` and writes the result into
/// `buf`.
#[inline]
pub fn escape_into(input: &str, buf: &mut String) {
    for c in input.chars() {
        match c {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            _ => buf.push(c),
        };
    }
}
pub struct Escape<T>(pub T);

impl Display for Escape<String> {
    #[inline]
    fn fmt(self, buf: &mut String) {
        escape_into(&self.0, buf)
    }
}

impl Display for Escape<&str> {
    #[inline]
    fn fmt(self, buf: &mut String) {
        escape_into(self.0, buf)
    }
}

impl Display for Escape<char> {
    #[inline]
    fn fmt(self, buf: &mut String) {
        escape_into(self.0.encode_utf8(&mut [0; 4]), buf)
    }
}

// impl<W, T> Display<W> for Escape<T>
// where
//     W: Write,
//     T: Display<W>,
// {
//     fn fmt(&self, buf: &mut W)
//     where
//         W: Write,
//     {
//         self.0.fmt(buf)
//     }
// }

/// A wrapper for pre-escaped types which shouldn't require further escaping.
pub struct PreEscaped<T: Sized>(pub T);

impl Display for PreEscaped<&str> {
    #[inline]
    fn fmt(self, buf: &mut String) {
        buf.push_str(self.0)
    }
}

impl Display for PreEscaped<String> {
    #[inline]
    fn fmt(self, buf: &mut String) {
        buf.push_str(&self.0)
    }
}

impl Display for PreEscaped<char> {
    #[inline]
    fn fmt(self, buf: &mut String) {
        buf.push(self.0)
    }
}
