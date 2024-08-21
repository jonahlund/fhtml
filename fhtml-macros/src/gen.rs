use fhtml_core::{escape::Escape, fmt::Display as _};
use fhtml_parser::ast;
use proc_macro2::TokenStream;
use quote::ToTokens as _;

pub(crate) type Output = String;
pub(crate) type Output2 = Vec<(usize, TokenStream)>;

pub(crate) struct Generator<'a> {
    output: &'a mut Output,
    output2: &'a mut Output2,
}

impl<'a, 'b> Generator<'a> {
    pub(crate) fn new(
        output: &'a mut Output,
        output2: &'a mut Output2,
    ) -> Self {
        Self { output, output2 }
    }

    fn write_dash_ident(&mut self, dash_ident: &ast::DashIdent) {
        for pair in dash_ident.0.pairs() {
            self.output.push_str(&pair.value().to_string());
            if pair.punct().is_some() {
                self.output.push('-');
            }
        }
    }

    fn write_doctype(&mut self, _: &ast::Doctype) {
        self.output.push_str("<!DOCTYPE html>");
    }

    fn write_expr(&mut self, expr: &syn::Expr) {
        match expr {
            syn::Expr::Lit(syn::ExprLit { lit, .. }) => match lit {
                syn::Lit::Str(lit_str) => {
                    Escape(lit_str.value()).fmt(self.output);
                }
                syn::Lit::Byte(lit_byte) => {
                    lit_byte.value().fmt(self.output);
                }
                syn::Lit::Char(lit_char) => {
                    Escape(lit_char.value()).fmt(self.output);
                }
                syn::Lit::Int(lit_int) => {
                    Escape(lit_int.base10_digits()).fmt(self.output);
                }
                syn::Lit::Float(lit_float) => {
                    Escape(lit_float.base10_digits()).fmt(self.output);
                }
                syn::Lit::Bool(lit_bool) => {
                    lit_bool.value().fmt(self.output);
                }
                _ => self
                    .output2
                    .push((self.output.len(), lit.to_token_stream())),
            },
            _ => {
                self.output2
                    .push((self.output.len(), expr.to_token_stream()));
            }
        }
    }

    fn write_value(&mut self, value: &ast::Value) {
        match value {
            ast::Value::LitStr(lit_str) => {
                self.output.push_str(&lit_str.value());
            }
            ast::Value::Expr(expr) => self.write_expr(expr),
        }
    }

    fn write_attribute(&mut self, attr: &'b ast::Attr) {
        self.output.push(' ');
        self.write_dash_ident(&attr.name);
        self.output.push('=');
        self.output.push('"');
        self.write_value(&attr.value);
        self.output.push('"');
    }

    fn write_tag(&mut self, tag: &ast::Tag) {
        match tag {
            ast::Tag::Opening { name, attrs, .. } => {
                self.output.push('<');
                self.write_dash_ident(name);
                for attr in attrs {
                    self.write_attribute(attr);
                }
                self.output.push('>');
            }
            ast::Tag::Closing { name } => {
                self.output.push_str("</");
                self.write_dash_ident(name);
                self.output.push('>');
            }
        }
    }

    fn write_node(&mut self, node: &ast::Node) {
        match node {
            ast::Node::Doctype(doctype) => self.write_doctype(doctype),
            ast::Node::Tag(tag) => self.write_tag(tag),
            ast::Node::Value(value) => self.write_value(value),
        }
    }
}

pub(crate) fn generate(nodes: &[ast::Node]) -> (Output, Output2) {
    let mut output = String::with_capacity(nodes.len() * 10);
    let mut output2 = Vec::with_capacity(nodes.len() * 3);

    let mut g = Generator::new(&mut output, &mut output2);

    for node in nodes {
        g.write_node(node);
    }

    (output, output2)
}
