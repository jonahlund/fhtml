#![allow(dead_code)]

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::Expr;

pub struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

pub struct Doctype {
    pub span: Span,
}

pub enum TagKind {
    Start {
        name: DashIdent,
        attributes: Vec<Attr>,
        self_closing: bool,
    },
    End {
        name: DashIdent,
    },
}

pub struct Tag {
    pub kind: TagKind,
    pub span: Span,
}

pub struct Attr {
    pub name: DashIdent,
    pub value: Value,
    pub span: Span,
}

#[derive(Clone)]
pub enum Value {
    Text(syn::LitStr),
    Braced {
        value: Expr,
        specs: Option<TokenStream>,
    },
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Value::Text(literal) => literal.to_tokens(tokens),
            Value::Braced { value, .. } => value.to_tokens(tokens),
        }
    }
}

pub enum Segment {
    Doctype(Doctype),
    Tag(Tag),
    Value(Value),
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Segment::Doctype(doctype) => doctype.to_string().to_tokens(tokens),
            Segment::Tag(tag) => tag.to_string().to_tokens(tokens),
            Segment::Value(value) => value.to_tokens(tokens),
        }
    }
}

pub struct Template {
    pub segments: Vec<Segment>,
    pub values: Vec<Value>,
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_string().to_tokens(tokens);
    }
}
