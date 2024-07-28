use core::fmt;

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
pub fn escape(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    let _ = escape_into(input, &mut buf);
    buf
}

/// Escapes all special HTML characters in `input` and writes the result into
/// `output`.
pub fn escape_into(input: &str, output: &mut impl fmt::Write) -> fmt::Result {
    for c in input.chars() {
        match c {
            '&' => output.write_str("&amp;")?,
            '<' => output.write_str("&lt;")?,
            '>' => output.write_str("&gt;")?,
            '"' => output.write_str("&quot;")?,
            _ => output.write_char(c)?,
        }
    }
    Ok(())
}

/// A wrapper for types that are pre-escaped and shouldn't require further
/// escaping.
pub struct PreEscaped<T>(pub T);

impl<T> Render for PreEscaped<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

pub trait Render {
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result;
}

impl Render for String {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        escape_into(self, f)
    }
}

impl Render for str {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        escape_into(self, f)
    }
}

impl Render for char {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        escape_into(self.encode_utf8(&mut [0; 4]), f)
    }
}

impl Render for bool {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(if *self { "true" } else { "false " })
    }
}

macro_rules! impl_itoa {
    ($($Ty:ty)*) => {
        $(
            impl Render for $Ty {
                #[inline]
                fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str(itoa::Buffer::new().format(*self))
                }
            }
        )*
    };
}

impl_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

macro_rules! impl_ryu {
    ($($Ty:ty)*) => {
        $(
            impl Render for $Ty {
                #[inline]
                fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str(ryu::Buffer::new().format(*self))
                }
            }
        )*
    };
}

impl_ryu! {
    f32 f64
}

impl Render for fmt::Arguments<'_> {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_fmt(*self)
    }
}

impl Render for () {
    #[inline]
    fn render_to(&self, _: &mut impl fmt::Write) -> fmt::Result {
        Ok(())
    }
}

impl<T> Render for &T
where
    T: ?Sized + Render,
{
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        Render::render_to(&**self, f)
    }
}
