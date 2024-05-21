use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;

pub struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);
pub struct Doctype;

pub enum Tag {
    Start {
        name: DashIdent,
        attributes: Vec<Attribute>,
        self_closing: bool,
    },
    End {
        name: DashIdent,
    },
}

pub struct Attribute {
    pub name: DashIdent,
    pub value: Value,
}

#[derive(Clone)]
pub enum Value {
    Text(syn::LitStr),
    Braced {
        value: syn::Expr,
        params: Option<TokenStream>,
        escape: bool,
    },
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Text(lit) => lit.to_tokens(tokens),
            Value::Braced { value, escape, .. } => {
                if *escape {
                    quote! {
                        ::fhtml::escape(#value)
                    }
                    .to_tokens(tokens)
                } else {
                    value.to_tokens(tokens)
                }
            }
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
