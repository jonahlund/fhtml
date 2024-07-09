use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::Expr;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Doctype;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum LitValue {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) enum PlaceholderValue {
    LitStr(syn::LitStr),
    Expr {
        value: Expr,
        specs: Option<TokenStream>,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Attr<Value: Parse> {
    pub(crate) name: DashIdent,
    pub(crate) value: Value,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Tag<Value: Parse> {
    Opening {
        name: DashIdent,
        attrs: Vec<Attr<Value>>,
        #[allow(dead_code)]
        self_closing: bool,
    },
    Closing {
        name: DashIdent,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Node<Value: Parse> {
    Doctype(Doctype),
    Tag(Tag<Value>),
    Value(Value),
}

impl ToTokens for LitValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitValue::LitStr(x) => x.to_tokens(tokens),
            LitValue::Expr(x) => x.to_tokens(tokens),
        }
    }
}

impl From<PlaceholderValue> for LitValue {
    fn from(value: PlaceholderValue) -> Self {
        match value {
            PlaceholderValue::LitStr(value) => Self::LitStr(value),
            PlaceholderValue::Expr { value, .. } => Self::Expr(value),
        }
    }
}

impl<Value: Parse + Clone> Node<Value> {
    pub(crate) fn get_all_values(&self) -> Vec<Value> {
        let mut v = Vec::new();

        match self {
            #[rustfmt::skip]
            Node::Tag(Tag::Opening { attrs, .. }) => for attr in attrs {
                v.push(attr.value.clone());
            },
            Node::Value(value) => {
                v.push(value.clone());
            }
            _ => {}
        }

        v
    }
}
