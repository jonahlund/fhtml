use proc_macro2::TokenStream;
use quote::ToTokens;

pub enum Value {
    Text(syn::LitStr),
    Braced(syn::Expr),
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Value::Text(lit) => lit.to_tokens(tokens),
            Value::Braced(expr) => expr.to_tokens(tokens),
        }
    }
}

pub struct Attribute {
    pub name: DashIdent,
}

pub struct DashIdent {
    pub value: String,
}

pub enum Segment {
    Value,
    OpeningTag {
        name: DashIdent,
        attributes: Vec<Attribute>,
    },
    ClosingTag {
        name: DashIdent,
    },
    SelfClosingTag {
        name: DashIdent,
        attributes: Vec<Attribute>,
    },
}

pub struct Html {
    pub segments: Vec<Segment>,
    pub values: Vec<Value>,
}
