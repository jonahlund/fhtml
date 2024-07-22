use core::fmt;

pub trait Render<W: fmt::Write> {
    fn render_to(self, w: &mut W) -> fmt::Result;
}

impl<W: fmt::Write> Render<W> for String {
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        for c in self.chars() {
            c.render_to(w)?;
        }
        Ok(())
    }
}

impl<W: fmt::Write> Render<W> for &String {
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        for c in self.chars() {
            c.render_to(w)?;
        }
        Ok(())
    }
}

impl<W: fmt::Write> Render<W> for &str {
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        for c in self.chars() {
            c.render_to(w)?;
        }
        Ok(())
    }
}

impl<W: fmt::Write> Render<W> for char {
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        match self {
            '&' => w.write_str("&amp;"),
            '<' => w.write_str("&lt;"),
            '>' => w.write_str("&gt;"),
            '"' => w.write_str("&quot;"),
            '\'' => w.write_str("&#39;"),
            _ => w.write_char(self),
        }
    }
}

impl<W: fmt::Write> Render<W> for bool {
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        w.write_str(if self { "true" } else { "false " })
    }
}

macro_rules! render_via_itoa {
    ($($Ty:ty)*) => {
        $(
            impl <W: fmt::Write> Render<W> for $Ty {
                #[inline]
                fn render_to(self, w: &mut W) -> fmt::Result {
                    w.write_str(itoa::Buffer::new().format(self))
                }
            }
        )*
    };
}

render_via_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

macro_rules! render_via_ryu {
    ($($Ty:ty)*) => {
        $(
            impl<W: fmt::Write> Render<W> for $Ty {
                #[inline]
                fn render_to(self, w: &mut W) -> fmt::Result {
                    w.write_str(ryu::Buffer::new().format(self))
                }
            }
        )*
    };
}

render_via_ryu! {
    f32 f64
}

impl<W: fmt::Write> Render<W> for fmt::Arguments<'_> {
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        w.write_fmt(self)
    }
}

impl<W, F> Render<W> for F
where
    W: fmt::Write,
    F: FnOnce(&mut W) -> fmt::Result,
{
    #[inline]
    fn render_to(self, w: &mut W) -> fmt::Result {
        self(w)
    }
}
