pub use fhtml_macros::*;

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
    fn it_works() {}
}
