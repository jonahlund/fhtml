use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

mod fmt;
mod html;
mod parse;

struct WriteInput {
    formatter: syn::Expr,
    template: html::Template,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let formatter = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let template = input.parse()?;

        Ok(Self {
            formatter,
            template,
        })
    }
}

/// Writes formatted HTML to a buffer.
///
/// `fhtml::write!` works similar to [`std::write!`] with a few key differences:
/// - HTML can be written as-is without having to be inside a string literal.
/// - Expressions are written directly inside braces, compared to
///   [`std::write!`], where they are passed as separate parameters.
///
/// Formatting specifiers are written after expressions, denoted by
/// a colon `:`, similar to how they are written in [`std::write!`].
///
/// Values are not escaped implicitly, but are opt-in with an exclamation mark
/// `!` preceding any formatting specifiers:
/// `{[expr]:![specifiers]}`.
///
/// [`std::write!`]: std::write
///
/// # Examples
///
/// ## Simple usage
///
/// ```ignore
/// let mut buffer = String::new();
/// let _ = fhtml::write!(buffer, <div>"Hello, World!"</div>);
/// assert_eq!(buffer,  "<div>Hello, World!</div>");
/// ```
///
/// ## Escaping values
///
/// ```ignore
/// let mut buffer = String::new();
/// let user_input = "<b>Yay</b>";
/// let _ = fhtml::write!(buffer, <div>{user_input:!}</div>);
/// assert_eq!(buffer, "<div>&lt;b&gt;Yay&lt;/b&gt;</div>");
/// ```
#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    let WriteInput {
        formatter,
        template,
    } = syn::parse_macro_input!(input as WriteInput);

    let values = &template.values;
    let template = template.to_string();

    let output = quote! {
        ::std::write!(#formatter, #template, #(#values),*)
    };

    output.into()
}

/// Creates a compile time `&'static str` with formatted HTML.
///
/// Since there are no suitable `std` macros for compile-time string formatting,
/// `fhtml::formatcp!` is the only `fhtml` macro that uses a dependency:
/// [`const_format`]. `fhtml::formatcp!` has the same syntax as
/// [`fhtml::write!`], with the biggest difference being: values are limited by
/// what can be evaluated at compile-time.
///
/// Read about [limitations and behaviours here](https://docs.rs/const_format/latest/const_format/macro.formatcp.html).
///
/// [`fhtml::write!`]: crate::write!
/// [`const_format`]: https://docs.rs/const_format
/// [`const_format::formatcp!`]: https://docs.rs/const_format/latest/const_format/macro.formatcp.html
///
/// # Examples
///
/// ```ignore
/// const HTML: &str = fhtml::formatcp!(<div>"Hello, World!"</div>);
/// ```
#[proc_macro]
pub fn formatcp(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as html::Template);

    let values = &template.values;
    let template = template.to_string();

    let output = quote! {
        ::fhtml::_internal::formatcp!(#template, #(#values),*)
    };

    output.into()
}
