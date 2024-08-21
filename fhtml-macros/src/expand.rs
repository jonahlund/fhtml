use fhtml_parser::ast;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{
    gen::generate,
    r#gen::{Output, Output2},
};

pub(crate) fn format_args(nodes: &[ast::Node]) -> TokenStream {
    let (output, output2) = generate(nodes);

    let arg_names: Vec<_> = output2
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("__arg{i}"))
        .collect();

    let output2_as_arg_names: Vec<_> = output2
        .iter()
        .enumerate()
        .map(|(i, (indice, _))| (*indice, arg_names[i].to_token_stream()))
        .collect();

    let stmts = expand_stmts(&output, &output2_as_arg_names);
    let decls = expand_decls(&output2);

    quote! {
        #(#decls;)*
        #[inline(always)]
        move |__buf: &mut String| {
            #(#stmts;)*
        }
    }
}

pub(crate) fn concat(nodes: &[ast::Node]) -> TokenStream {
    let (output, output2) = generate(nodes);
    assert_eq!(output2.len(), 0);

    quote! {
        ::fhtml::PreEscaped(#output)
    }
}

fn expand_str_stmt(val: &str) -> TokenStream {
    quote! {
        __buf.push_str(#val)
    }
}

fn expand_arg_stmt<T: ToTokens>(val: &T) -> TokenStream {
    quote! {
        ::fhtml::Display::fmt(::fhtml::Escape(#val), __buf)
    }
}

fn expand_stmts(output: &Output, output2: &Output2) -> Vec<TokenStream> {
    let mut stmts = Vec::new();
    let mut cursor = 0usize;

    for (i, arg) in output2 {
        if *i != cursor {
            stmts.push(expand_str_stmt(&output[cursor..*i]));
            cursor = *i;
        }
        stmts.push(expand_arg_stmt(&arg));
    }

    if cursor < output.len() {
        stmts.push(expand_str_stmt(&output[cursor..]));
    }

    stmts
}

// let __arg0 = (..)
fn expand_decls(output2: &Output2) -> Vec<TokenStream> {
    output2
        .iter()
        .enumerate()
        .map(|(i, (_, val))| {
            let name = format_ident!("__arg{i}");

            quote! {
                let #name = #val
            }
        })
        .collect()
}
