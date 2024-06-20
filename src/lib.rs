//! fhtml - Rust formatting macros for HTML
//!
//! `fhtml` builds upon two fundamental macros: `format_args!` and
//! `render_args!`. `format_args!` can be seen as the `std-compatible` part of
//! `fhtml`, providing familiar macros such as `write!`, `format!`, `println!`
//! etc. `render_args!` provides more low-level control, but lacks the
//!
//! Every other macro is proxied through either one of those

pub use fhtml_macros::*;

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! writeln {
    () => {};
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = ::std::fmt::format($crate::format_args!($($arg)*));
        res
    }};
}

#[macro_export]
macro_rules! print {
    () => {};
}

#[macro_export]
macro_rules! println {
    () => {};
}

#[macro_export]
macro_rules! eprint {
    () => {};
}

#[macro_export]
macro_rules! eprintln {
    () => {};
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    #[test]
    fn it_works() {
        let mut output = String::new();

        let _ = self::write!(output, <div>"Hello, world!"</div>);

        assert_eq!(output, "<div>Hello, world!</div>");
    }
}
