use syn::punctuated::Punctuated;

/// An identifier separated by dashes: `foo-bar-baz`.
#[derive(PartialEq, Clone, Debug)]
pub struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

// An HTML doctype: `<!DOCTYPE html>`.
#[derive(PartialEq, Clone, Debug)]
pub struct Doctype;

/// A value that is either a string literal or an expression: `"foo"`,
/// `1 + 2`.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

/// An HTML attribute: `foo="bar"`
#[derive(Clone, Debug, PartialEq)]
pub struct Attr {
    pub name: DashIdent,
    pub value: Value,
}

/// An HTML opening or closing tag: `<foo ..>`, `</bar>`.
#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Opening {
        name: DashIdent,
        attrs: Vec<Attr>,
        void: bool,
    },
    Closing {
        name: DashIdent,
    },
}

pub enum Node {
    Doctype(Doctype),
    Tag(Tag),
    Value(Value),
}
