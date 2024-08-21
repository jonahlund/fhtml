pub trait Display {
    fn fmt(self, buf: &mut String);
}

impl Display for bool {
    #[inline]
    fn fmt(self, buf: &mut String) {
        buf.push_str(if self { "true" } else { "false" })
    }
}

macro_rules! impl_via_itoa {
    ($($Ty:ty)*) => {
        $(
            impl Display for $Ty {
                #[inline]
                fn fmt(self, buf: &mut String) {
                    buf.push_str(itoa::Buffer::new().format(self))
                }
            }
        )*
    };
}

impl_via_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

macro_rules! impl_via_ryu {
    ($($Ty:ty)*) => {
        $(
            impl Display for $Ty {
                #[inline]
                fn fmt(self, buf: &mut String) {
                    buf.push_str(ryu::Buffer::new().format(self))
                }
            }
        )*
    };
}

impl_via_ryu! {
    f32 f64
}

pub trait Debug {
    fn fmt(&self, output: &mut String);
}

pub trait Octal {}

pub trait LowerHex {}

pub trait UpperHex {}

pub trait Pointer {}

pub trait Binary {}

pub trait LowerExp {}

pub trait UpperExp {}

pub struct FormatterFn<F>(F);

impl<F> Display for FormatterFn<F>
where
    F: FnOnce(&mut String),
{
    fn fmt(self, buf: &mut String) {
        (self.0)(buf)
    }
}

#[inline]
pub fn from_fn<F>(f: F) -> FormatterFn<F>
where
    F: Fn(&mut String),
{
    FormatterFn(f)
}
