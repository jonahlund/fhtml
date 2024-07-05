//! fhtml - Rust formatting macros for HTML.
//!
//! `fhtml` provides familiar formatting macros such as `write!`, `format!`,
//! `println!`, `concat!`, etc, but tailored for HTML.
//!
//! Escaping of values is done manually.

pub use fhtml_macros::*;

/// Write formatted HTML to a buffer.
///
/// See [`std::write!`] for more information.
///
/// [`std::write!`]: https://doc.rust-lang.org/stable/std/macro.write.html
#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// Writes formatted HTML to a buffer with a newline (`<br>`) appended.
///
/// See [`std::writeln!`] for more information.
///
/// [`std::writeln!`]: https://doc.rust-lang.org/stable/std/macro.writeln.html
#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, <br />)
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args_nl!($($arg)*))
    };
}

/// Writes formatted HTML with embedded expressions to a `String`.
///
/// See [`std::format!`] for more information.
///
/// [`std::format!`]: https://doc.rust-lang.org/stable/std/macro.format.html
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = ::std::fmt::format($crate::format_args!($($arg)*));
        res
    }};
}

/// Escapes special HTML characters in a string.
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
    use std::fmt::Write;

    #[test]
    fn write() {
        let mut output = String::new();
        let _ = crate::write!(output, <h1>"Hello, world!"</h1>);
        assert_eq!(output, "<h1>Hello, world!</h1>");
    }

    #[test]
    fn writeln() {
        let mut output = String::new();
        let _ = crate::writeln!(output, <h1>"Hello, world!"</h1>);
        assert_eq!(output, "<h1>Hello, world!</h1><br>");
    }

    #[test]
    fn format() {
        assert_eq!(format!(<h1>"Hello, world!"</h1>), "<h1>Hello, world!</h1>");
    }

    #[test]
    fn concat() {
        assert_eq!(crate::concat!(<div>{1}</div>), "<div>1</div>");
    }
}
