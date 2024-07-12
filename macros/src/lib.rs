use proc_macro::TokenStream;
use quote::quote;

mod analyze;
mod ast;
mod fmt;
mod lower_ast;
mod parse;

pub(crate) struct FormatArgsInput {
    pub fmt: String,
    pub args: Vec<ast::ArgValue>,
}

/// A low level macro for creating an [`fmt::Arguments`] with formatted HTML.
///
/// See [`std::format_args!`] for more information.
///
/// [`fmt::Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html
/// [`std::format_args!`]: https://doc.rust-lang.org/stable/std/macro.format_args.html
#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    let FormatArgsInput { fmt, args } =
        syn::parse_macro_input!(input as FormatArgsInput);

    let output = quote! {
        ::std::format_args!(#fmt, #(#args),*)
    };

    output.into()
}

/// Creates an [`fmt::Arguments`] with a newline (`<br>`) appended.
///
/// See [`std::format_args_nl!`] for more information.
///
/// [`fmt::Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html
/// [`std::format_args_nl!`]: https://doc.rust-lang.org/stable/std/macro.format_args_nl.html
#[proc_macro]
pub fn format_args_nl(input: TokenStream) -> TokenStream {
    let FormatArgsInput {
        fmt: template,
        args: values,
    } = syn::parse_macro_input!(input as FormatArgsInput);

    let template_with_nl = format!("{}<br>", template);

    let output = quote! {
        ::std::format_args!(#template_with_nl, #(#values),*)
    };

    output.into()
}

pub(crate) struct ConcatInput {
    pub segments: Vec<proc_macro2::TokenStream>,
}

/// Creates a `&'static str` with formatted HTML.
///
/// Only supports certain literals as values.
/// See [`std::concat!`] for more information.
///
/// [`std::concat!`]: https://doc.rust-lang.org/stable/std/macro.concat.html
#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let ConcatInput { segments } =
        syn::parse_macro_input!(input as ConcatInput);

    let output = quote! {
        ::std::concat!(#(#segments),*)
    };

    output.into()
}
