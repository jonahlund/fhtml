use syn::punctuated::Punctuated;

// An identifier separated by dashes, `foo-bar-baz`.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

// An HTML doctype, `<!DOCTYPE html>`.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Doctype;

/// A value that is either a string literal or an expression.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

/// An HTML attribute, foo="bar"
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Attr {
    pub name: DashIdent,
    pub value: Value,
}

/// An HTML opening or closing tag, <foo>, </bar>.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Tag {
    Opening {
        name: DashIdent,
        attrs: Vec<Attr>,
        self_closing: bool,
    },
    Closing {
        name: DashIdent,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Node {
    Doctype(Doctype),
    Tag(Tag),
    Value(Value),
}

impl quote::ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::LitStr(lit_str) => lit_str.to_tokens(tokens),
            Value::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}
