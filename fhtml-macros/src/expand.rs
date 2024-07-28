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

    let scope =
        syn::Lifetime::new(&format!("'_scope_{:x}", digest), Span::call_site());

    let ops = merge(&slice(nodes)).into_iter().map(|s| match s {
        Segment::String(val) => quote! {
            if let Err(e) = #buffer.write_str(#val) {
                break #scope Err(e);
            }
        },
        Segment::Tokens(val) => quote! {
            if let Err(e) = ::fhtml::Render::render_to(&(#val), &mut #buffer) {
                break #scope Err(e);
            }
        },
    });

    quote! {
        #scope: {
            #buffer.reserve(#size);
            #(#ops)*
            ::core::fmt::Result::Ok(())
        }
    }
}

pub(crate) fn format(input: FormatInput, size: usize) -> TokenStream {
    let FormatInput { nodes } = input;

    let buffer = syn::Ident::new("_buffer", Span::call_site());

    let ops = merge(&slice(nodes)).into_iter().map(|s| match s {
        Segment::String(val) => quote! {
            #buffer.push_str(#val);
        },
        Segment::Tokens(val) => quote! {
            let _ = ::fhtml::Render::render_to(&(#val), &mut #buffer);
        },
    });

    quote! {
        {
            let mut #buffer = String::with_capacity(#size);
            #(#ops)*
            ::fhtml::PreEscaped(#buffer)
        }
    }
}

pub(crate) fn concat(input: ConcatInput) -> TokenStream {
    let ConcatInput { nodes } = input;

    let literal = merge(&slice(nodes)).into_iter().fold(
        String::new(),
        |acc, s| match s {
            Segment::String(val) => acc + &val,
            Segment::Tokens(_) => todo!(),
        },
    );

    quote! {
        ::fhtml::PreEscaped(#literal)
    }
}

enum Segment {
    String(String),
    Tokens(TokenStream),
}

fn merge(parts: &[ast::Part]) -> Vec<Segment> {
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

fn slice(nodes: Vec<ast::Node>) -> Vec<ast::Part> {
    nodes
        .into_iter()
        .flat_map(|n| n.into_parts())
        .collect::<Vec<_>>()
}
