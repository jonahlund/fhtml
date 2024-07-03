//! fhtml - Rust formatting macros for HTML
//!
//! `fhtml` provides std-compatible formatting macros such as `write!`,
//! `format!`, `println!`, `concat!`, etc, but tailored for HTML-like syntax

pub use fhtml_macros::*;

/// Write formatted HTML to a buffer.
///
/// `write!` is identical to `std::write!`
#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// Write formatted HTML to a buffer with a newline (`<br>`) appended.
#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, <br>)
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args_nl!($($arg)*))
    };
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = ::std::fmt::format($crate::format_args!($($arg)*));
        res
    }};
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
