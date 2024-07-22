pub use fhtml_macros::*;

pub use self::alloc::*;

mod alloc;

#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, <br />)
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args_nl!($($arg)*))
    };
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {
        {
            use ::core::fmt::Write as _;
            let mut w = String::new();
            let _ = $crate::write!(w, $($arg)*);
            w
        }
    };
}
