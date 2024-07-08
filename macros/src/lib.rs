use proc_macro::TokenStream;
use quote::quote;

mod ast;
mod fmt;
mod lower_ast;
mod parse;

pub(crate) struct FormatArgsInput {
    pub template: String,
    pub values: Vec<ast::LitValue>,
}

/// A low level macro for creating an [`fmt::Arguments`] with formatted HTML.
///
/// See [`std::format_args!`] for more information.
///
/// [`fmt::Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html
/// [`std::format_args!`]: https://doc.rust-lang.org/stable/std/macro.format_args.html
#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as FormatArgsInput);

    let template = input.template;
    let values = input.values;

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
    let input = syn::parse_macro_input!(input as FormatArgsInput);

    let template_with_nl = format!("{}<br>", input.template);
    let values = input.values;

    let output = quote! {
        ::std::format_args!(#template_with_nl, #(#values),*)
    };

    output.into()
}

pub(crate) struct ConcatInput {
    pub segments: Vec<proc_macro2::TokenStream>,
}

/// Creates a &'static str with formatted HTML.
///
/// Only supports certain literals as values.
/// See [`std::concat!`] for more information.
///
/// [`std::concat!`]: https://doc.rust-lang.org/stable/std/macro.concat.html
#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ConcatInput);
    let segments = input.segments;

    let output = quote! {
        ::std::concat!(#(#segments),*)
    };

    output.into()
}
