use proc_macro::TokenStream;
use quote::quote;

mod analyze;
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

#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let template = syn::parse_macro_input!(input as ast::Template);
    let segments = &template.segments;

    let output = quote! {
        ::std::concat!(#(#segments),*)
    };

    output.into()
}
