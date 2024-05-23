use proc_macro::TokenStream;
use quote::quote;

mod ast;
mod fmt;
mod parse;

#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);
    let values = &template.values;

    let output = quote! {
        ::std::format_args!(#template, #(#values),*)
    };

    output.into()
}

/// Creates a compile time `&'static str` with formatted HTML.
///
/// Since there are no suitable `std` macros for compile-time string formatting,
/// `fhtml::const_format!` is the only `fhtml` macro that uses a dependency:
/// [`const_format`]. `fhtml::const_format!` has the same syntax as
/// [`fhtml::write!`], with the biggest difference being: values are limited by
/// what can be evaluated at compile-time.
///
/// Read about [limitations and behaviours here](https://docs.rs/const_format/latest/const_format/macro.const_format.html).
///
/// [`fhtml::write!`]: crate::write!
/// [`const_format`]: https://docs.rs/const_format
/// [`const_format::const_format!`]: https://docs.rs/const_format/latest/const_format/macro.const_format.html
///
/// # Examples
///
/// ```ignore
/// const HTML: &str = fhtml::const_format!(<div>"Hello, World!"</div>);
/// ```
#[proc_macro]
pub fn const_format(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);

    let values = &template.values;
    let template = template.to_string();

    let output = quote! {
        ::fhtml::_internal::formatcp!(#template, #(#values),*)
    };

    output.into()
}
