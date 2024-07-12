use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

// An identifier separated by dashes, `foo-bar-baz`.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Doctype;

/// A value that is either a string literal or an expression.
///
/// This is the most straight-forward value type, used in `fhtml::concat!` and
/// tests.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LitValue {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

/// A value that represents an argument, usually used in formatting contexts.
///
/// The string representation of this value is a placeholder `{}`, that may
/// contain formatting specifiers `{:?}`.
///
/// The token representation of this value is the actual Rust value it contains,
/// either `LitStr` or `Expr`.
#[derive(Clone, Debug)]
pub(crate) enum ArgValue {
    LitStr(syn::LitStr),
    Expr {
        value: syn::Expr,
        specs: Option<TokenStream>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Attr<V> {
    pub name: DashIdent,
    pub eq_sep: syn::token::Eq,
    pub value: V,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Tag<V> {
    Opening {
        name: DashIdent,
        attrs: Vec<Attr<V>>,
        self_closing_slash: Option<syn::token::Slash>,
    },
    Closing {
        name: DashIdent,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Node<V> {
    Doctype(Doctype),
    Tag(Tag<V>),
    Value(V),
}

impl DashIdent {
    pub(crate) fn span(&self) -> Span {
        self.0.span()
    }
}

impl<V: Spanned> Attr<V> {
    pub(crate) fn span(&self) -> Span {
        join_spans([self.name.span(), self.eq_sep.span(), self.value.span()])
    }
}

impl<V> Tag<V> {
    pub(crate) fn name(&self) -> &DashIdent {
        match self {
            Self::Opening { name, .. } => name,
            Self::Closing { name } => name,
        }
    }

    pub(crate) fn is_self_closing(&self) -> bool {
        matches!(
            self,
            Self::Opening {
                self_closing_slash: Some(_),
                ..
            }
        )
    }
}

impl<V: Spanned> Tag<V> {
    pub(crate) fn span(&self) -> Span {
        match self {
            Tag::Opening {
                name,
                attrs,
                self_closing_slash,
            } => {
                let mut v = Vec::with_capacity(2 + attrs.len());
                v.push(name.span());
                for attr in attrs {
                    v.push(attr.span());
                }
                if let Some(slash) = self_closing_slash {
                    v.push(slash.span());
                }
                join_spans(v)
            }
            Tag::Closing { name } => name.span(),
        }
    }
}

impl<V: Clone> Node<V> {
    pub(crate) fn get_all_values(&self) -> Vec<V> {
        let mut v = Vec::new();

        match self {
            Node::Tag(Tag::Opening { attrs, .. }) => {
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

impl ToTokens for ArgValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ArgValue::LitStr(value) => value.to_tokens(tokens),
            ArgValue::Expr { value, .. } => value.to_tokens(tokens),
        }
    }
}

fn join_spans(spans: impl IntoIterator<Item = Span>) -> Span {
    let mut iter = spans.into_iter();

    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };

    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}
