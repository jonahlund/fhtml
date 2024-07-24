use core::fmt;

pub trait Render {
    fn render_to(&self, buffer: &mut impl fmt::Write) -> fmt::Result;
}

impl Render for String {
    #[inline]
    fn render_to(&self, buffer: &mut impl fmt::Write) -> fmt::Result {
        str::render_to(self, buffer)
    }
}

impl Render for str {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for c in self.chars() {
            c.render_to(f)?;
        }
        Ok(())
    }
}

impl Render for char {
    #[inline]
    fn render_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self {
            '&' => f.write_str("&amp;"),
            '<' => f.write_str("&lt;"),
            '>' => f.write_str("&gt;"),
            '"' => f.write_str("&quot;"),
            _ => f.write_char(*self),
        }
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
            impl ToHtml for $Ty {
                #[inline]
                fn to_html(&self, f: &mut impl fmt::Write) -> fmt::Result {
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
            impl ToHtml for $Ty {
                #[inline]
                fn to_html(&self, f: &mut impl fmt::Write) -> fmt::Result {
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
