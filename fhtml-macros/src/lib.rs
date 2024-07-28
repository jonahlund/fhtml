#![cfg_attr(feature = "nightly", feature(proc_macro_expand))]

use std::hash::{DefaultHasher, Hash, Hasher};

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod ast;
mod expand;
mod fmt;
mod parse;

pub(crate) struct WriteInput {
    buffer: syn::Expr,
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    let input_str = rm_whitespace(&input.to_string());
    let input = parse_macro_input!(input as WriteInput);

    expand::write(input, input_str.len(), hash(&input_str)).into()
}

pub(crate) struct FormatInput {
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn format(input: TokenStream) -> TokenStream {
    let input_str = rm_whitespace(&input.to_string());
    let input = parse_macro_input!(input as FormatInput);

    expand::format(input, input_str.len()).into()
}

pub(crate) struct ConcatInput {
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ConcatInput);

    expand::concat(input).into()
}

fn rm_whitespace(input: &str) -> String {
    input.replace(' ', "")
}

fn hash<V: Hash>(value: &V) -> usize {
    let mut hasher = DefaultHasher::new();

    Hash::hash(value, &mut hasher);
    Hasher::finish(&hasher) as usize
}
