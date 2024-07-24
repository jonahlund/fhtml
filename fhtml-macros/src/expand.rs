use std::{fmt::Write, mem};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens as _};

use crate::{ast, ConcatInput, FormatInput, WriteInput};

pub(crate) fn write(
    input: WriteInput,
    size: usize,
    digest: usize,
) -> TokenStream {
    let WriteInput { buffer, nodes } = input;

    let writer = syn::Ident::new("_writer", Span::call_site());
    let scope =
        syn::Lifetime::new(&format!("'_scope_{:x}", digest), Span::call_site());

    let ops = accumulate(&shatter(nodes)).into_iter().map(|s| match s {
        Segment::String(val) => quote! {
            if let Err(e) = #writer.write_str(#val) {
                break #scope Err(e);
            }
        },
        Segment::Tokens(val) => quote! {
            if let Err(e) = ::fhtml::ToHtml::to_html(&(#val), #writer) {
                break #scope Err(e);
            }
        },
    });

    quote! {
        #scope: {
            let #writer = &mut (#buffer);
            #writer.reserve(#size);
            #(#ops)*
            ::core::fmt::Result::Ok(())
        }
    }
}

pub(crate) fn format(input: FormatInput, size: usize) -> TokenStream {
    let FormatInput { nodes } = input;

    let writer = syn::Ident::new("_writer", Span::call_site());

    let ops = accumulate(&shatter(nodes)).into_iter().map(|s| match s {
        Segment::String(val) => quote! {
            #writer.push_str(#val);
        },
        Segment::Tokens(val) => quote! {
            let _ = ::fhtml::ToHtml::to_html(&(#val), &mut buf);
        },
    });

    quote! {
        {
            let mut #writer = String::with_capacity(#size);
            #(#ops)*
            buf
        }
    }
}

pub(crate) fn concat(input: ConcatInput) -> TokenStream {
    let ConcatInput { nodes } = input;

    let literal = accumulate(&shatter(nodes)).into_iter().fold(
        String::new(),
        |acc, s| match s {
            Segment::String(val) => acc + &val,
            // This should always return an error here, even though nightly
            // has proc_macro_expand, this should resolve at
            // the segment parser and not here, thus any
            // possible macro expansions should yield a String
            // segment
            Segment::Tokens(_) => todo!(),
        },
    );

    quote! {
        #literal
    }
}

enum Segment {
    String(String),
    Tokens(TokenStream),
}

fn accumulate(parts: &[ast::Part]) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut acc = String::new();

    for part in parts {
        if let ast::Part::AttrValue(val) | ast::Part::Value(val) = part {
            if write!(acc, "{}", val).is_err() {
                segments.push(Segment::String(mem::take(&mut acc)));
                segments.push(Segment::Tokens(val.to_token_stream()));
            }
        } else {
            let _ = write!(acc, "{}", part);
        }
    }

    if !acc.is_empty() {
        segments.push(Segment::String(acc))
    }

    segments
}

fn shatter(nodes: Vec<ast::Node>) -> Vec<ast::Part> {
    nodes
        .into_iter()
        .flat_map(|n| n.into_parts())
        .collect::<Vec<_>>()
}
