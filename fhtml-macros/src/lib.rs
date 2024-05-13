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
