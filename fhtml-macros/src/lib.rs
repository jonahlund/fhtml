mod expand;
mod gen;
mod parse;

use fhtml_parser::ast;
use proc_macro::TokenStream;
use syn::parse_macro_input;

pub(crate) struct FormatArgsInput {
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    let FormatArgsInput { nodes } =
        parse_macro_input!(input as FormatArgsInput);

    expand::format_args(&nodes).into()
}

pub(crate) struct ConcatInput {
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    let ConcatInput { nodes } = parse_macro_input!(input as ConcatInput);

    expand::concat(&nodes).into()
}
