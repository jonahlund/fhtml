use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;

pub struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);
pub struct Doctype;

pub enum Tag {
    Start {
        name: DashIdent,
        attributes: Vec<Attr>,
        self_closing: bool,
    },
    End {
        name: DashIdent,
    },
}

pub struct Attr {
    pub name: DashIdent,
    pub value: Value,
}

#[derive(Clone)]
pub struct BracedValue {
    pub value: syn::Expr,
    pub specs: Option<TokenStream>,
    pub escape: bool,
}

impl ToTokens for BracedValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let value = &self.value;

        if self.escape {
            quote! {
                ::fhtml::escape(#value)
            }
            .to_tokens(tokens)
        } else {
            value.to_tokens(tokens)
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Text(syn::LitStr),
    Braced(BracedValue),
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Text(value) => value.to_tokens(tokens),
            Value::Braced(value) => value.to_tokens(tokens),
        }
    }
}

pub enum Segment {
    Doctype(Doctype),
    Tag(Tag),
    Value(Value),
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
