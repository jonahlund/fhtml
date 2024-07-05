use proc_macro::TokenStream;
use quote::quote;

mod analyze;
mod ast;
mod fmt;
mod parse;

/// Creates an [`fmt::Arguments`].
///
/// See [`std::format_args!`] for more information.
///
/// [`fmt::Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html
/// [`std::format_args!`]: https://doc.rust-lang.org/stable/std/macro.format_args.html
#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);
    let values = &template.values;

    let output = quote! {
        ::std::format_args!(#template, #(#values),*)
    };

    output.into()
}

/// Creates an [`fmt::Arguments`] with a newline (`<br>`) appended.
///
/// See [`std::format_args_nl!`] for more information.
///
/// [`std::format_args_nl!`]: https://doc.rust-lang.org/stable/std/macro.format_args_nl.html
#[proc_macro]
pub fn format_args_nl(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);
    let values = &template.values;

    let template_with_nl = format!("{}<br>", template);

    let output = quote! {
        ::std::format_args!(#template_with_nl, #(#values),*)
    };

    output.into()
}

/// Creates an &'static str with the formatted HTML.
///
/// Only supports certain literals as values.
/// See [`std::concat!`] for more information.
///
/// [`std::concat!`]: https://doc.rust-lang.org/stable/std/macro.concat.html
#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);
    let segments = &template.segments;

    let output = quote! {
        ::std::concat!(#(#segments),*)
    };

    output.into()
}
