use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

mod display;
mod entity;
mod parser;

struct WriteInput {
    formatter: syn::Expr,
    html: entity::Html,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let formatter = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let html = input.parse()?;

        Ok(Self { formatter, html })
    }
}

#[proc_macro]
pub fn write(args: TokenStream) -> TokenStream {
    let WriteInput { formatter, html } =
        syn::parse_macro_input!(args as WriteInput);

    let values = &html.values;
    let html = html.to_string();

    let output = quote! {
        ::std::write!(#formatter, #html, #(#values),*)
    };
    output.into()
}

struct ConcatInput {
    html: entity::Html,
}

impl Parse for ConcatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html = input.parse()?;

        Ok(Self { html })
    }
}

#[proc_macro]
pub fn concat(args: TokenStream) -> TokenStream {
    let ConcatInput { html } = syn::parse_macro_input!(args as ConcatInput);

    let values = &html.values;
    let html = html.to_string();

    let output = quote! {
        ::fhtml::const_format::formatcp!(#html, #(#values),*)
    };
    output.into()
}
