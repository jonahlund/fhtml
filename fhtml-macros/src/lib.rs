use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

mod ast;
mod fmt;
mod hash;
mod lower_ast;
mod parse;
mod segment;

use self::hash::hash;
use self::segment::Segment;

pub(crate) struct WriteInput {
    buffer: syn::Expr,
    segments: Vec<Segment>,
}

#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let WriteInput { buffer, segments } =
        syn::parse_macro_input!(input as WriteInput);

    let input_size = input_str.len();
    let input_digest = hash(&input_str);

    let writer_ident = syn::Ident::new("_writer", Span::call_site());
    let scope_ident = syn::Lifetime::new(
        &format!("'_scope_{:x}", input_digest),
        Span::call_site(),
    );

    let ops = segments.into_iter().map(|s| match s {
        Segment::String(val) => quote! {
            if let Err(e) = #writer_ident.write_str(#val) {
                break #scope_ident Err(e);
            }
        },
        Segment::Tokens(val) => quote! {
            if let Err(e) = ::fhtml::Render::render_to(#val, #writer_ident) {
                break #scope_ident Err(e);
            }
        },
    });

    quote! {
        #scope_ident: {
            let #writer_ident = &mut (#buffer);
            #writer_ident.reserve(#input_size);
            #(#ops)*
            ::core::fmt::Result::Ok(())
        }
    }
    .into()
}

pub(crate) struct ConcatInput {
    pub segments: Vec<proc_macro2::TokenStream>,
}

#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let ConcatInput { segments } =
        syn::parse_macro_input!(input as ConcatInput);

    let output = quote! {
        ::std::concat!(#(#segments),*)
    };

    output.into()
}
