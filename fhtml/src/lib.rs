#![no_std]

extern crate alloc;

pub use fhtml_core::{
    escape::*,
    fmt,
    fmt::{from_fn, Display},
};
pub use fhtml_macros::{concat, format_args};

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {
        ::fhtml::Display::to_string(&::fhtml::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! write {
    ($buf:expr, $($arg:tt)*) => {
        ::fhtml::Display::fmt(&::fhtml::format_args!($($arg)*), $buf)
    };
}
