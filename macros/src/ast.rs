#![allow(dead_code)]

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::punctuated::Punctuated;

// An identifier separated by dashes, `foo-bar-baz`.
#[derive(Clone, Debug)]
pub(crate) struct DashIdent {
    pub inner: Punctuated<syn::Ident, syn::Token![-]>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub(crate) struct Doctype {
    pub span: Span,
}

/// This type is generic in order to support multiple kinds of values.
#[derive(Clone, Debug)]
pub(crate) struct Value<V> {
    pub inner: V,
    pub span: Span,
}

/// A value that is either a string literal or an expression.
///
/// This is the most straight-forward value type, used in `fhtml::concat!` and
/// tests.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LitValue {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

/// A value that represents a placeholder, usually used in formatting contexts.
///
/// The string representation of this value is a placeholder `{}`, that may
/// contain formatting specifiers `{:?}`.
///
/// The token representation of this value is the actual Rust value it contains,
/// either `LitStr` or `Expr`.
#[derive(Clone, Debug)]
pub(crate) enum PlaceholderValue {
    LitStr(syn::LitStr),
    Expr {
        value: syn::Expr,
        specs: Option<TokenStream>,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct Attr<V> {
    pub name: DashIdent,
    pub value: Value<V>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TagKind<V> {
    Opening {
        name: DashIdent,
        attrs: Vec<Attr<V>>,
        self_closing: bool,
    },
    Closing {
        name: DashIdent,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct Tag<V> {
    pub kind: TagKind<V>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Node<V> {
    Doctype(Doctype),
    Tag(Tag<V>),
    Value(Value<V>),
}

impl<V> Tag<V> {
    pub(crate) fn name(&self) -> &DashIdent {
        match &self.kind {
            TagKind::Opening { name, .. } => name,
            TagKind::Closing { name } => name,
        }
    }
}

impl<V: Clone> Node<V> {
    pub(crate) fn get_all_values(&self) -> Vec<Value<V>> {
        let mut v = Vec::new();

        match self {
            Node::Tag(Tag {
                kind: TagKind::Opening { attrs, .. },
                ..
            }) => {
                for attr in attrs {
                    v.push(attr.value.clone());
                }
            }
            Node::Value(value) => {
                v.push(value.clone());
            }
            _ => {}
        }

        v
    }
}

impl ToTokens for LitValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitValue::LitStr(value) => value.to_tokens(tokens),
            LitValue::Expr(value) => value.to_tokens(tokens),
        }
    }
}

impl ToTokens for PlaceholderValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PlaceholderValue::LitStr(value) => value.to_tokens(tokens),
            PlaceholderValue::Expr { value, .. } => value.to_tokens(tokens),
        }
    }
}

impl<V: ToTokens> ToTokens for Value<V> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

impl PartialEq for DashIdent {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq for Doctype {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<V: PartialEq> PartialEq for Value<V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<V: PartialEq> PartialEq for Attr<V> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl<V: PartialEq> PartialEq for Tag<V> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl<V> From<Doctype> for Node<V> {
    fn from(value: Doctype) -> Self {
        Self::Doctype(value)
    }
}

impl<V> From<Tag<V>> for Node<V> {
    fn from(value: Tag<V>) -> Self {
        Self::Tag(value)
    }
}
