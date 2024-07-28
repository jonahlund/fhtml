use quote::ToTokens;
use syn::punctuated::Punctuated;

/// An identifier separated by dashes, e.g., `foo-bar-baz`.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

// An HTML doctype, `<!DOCTYPE html>`.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Doctype;

/// A value that is either a string literal or an expression, e.g., `"foo"`,
/// `1 + 2`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

/// An HTML attribute, e.g., `foo="bar"`
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Attr {
    pub name: DashIdent,
    pub value: Value,
}

/// An HTML opening or closing tag, e.g., `<foo ..>`, `</bar>`.
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

/// An HTML doctype, tag or value
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Node {
    Doctype(Doctype),
    Tag(Tag),
    Value(Value),
}

/// A small, fine grained representation of an HTML node.
///
/// This is useful for token processing, formatting, and more, where a complete
/// HTML node representation might otherwise be limiting and insufficient.
#[derive(Debug, PartialEq)]
pub(crate) enum Part {
    /// An HTML doctype, e.g., `<!DOCTYPE html>`.
    Doctype,

    // Opening tag, e.g., `<foo ..>`
    /// The beginning of an opening tag, `<`.
    OpeningTagStart,
    /// The opening tag name.
    OpeningTagName(DashIdent),
    /// The ending of an opening tag, `>`.
    OpeningTagEnd,

    // Closing tag, e.g., `</foo>`.
    /// The beginning of a closing tag, `</`.
    ClosingTagStart,
    /// The closing tag name.
    ClosingTagName(DashIdent),
    /// The ending of a closing tag, `>`.
    ClosingTagEnd,

    // Attribute, e.g., `foo="bar"`.
    /// A whitespace preceeding the attribute.
    AttrSpace,
    /// The attribute name.
    AttrName(DashIdent),
    /// An equal sign `=`, separating the attribute name from the value.
    AttrEqSep,
    /// The opening quote, `"`.
    AttrValueOpeningQuote,
    /// The attribute value.
    AttrValue(Value),
    /// The closing quote, `"`.
    AttrValueClosingQuote,

    // A standalone value, i.e., a value not part of a tag definition.
    Value(Value),
}

impl Attr {
    /// Converts an HTML attribute into smaller parts.
    pub(crate) fn into_parts(self) -> [Part; 6] {
        [
            Part::AttrSpace,
            Part::AttrName(self.name),
            Part::AttrEqSep,
            Part::AttrValueOpeningQuote,
            Part::AttrValue(self.value),
            Part::AttrValueClosingQuote,
        ]
    }
}

impl Tag {
    /// Converts an HTML tag into smaller parts.
    pub(crate) fn into_parts(self) -> Vec<Part> {
        match self {
            Self::Opening { name, attrs, .. } => {
                let mut v =
                    vec![Part::OpeningTagStart, Part::OpeningTagName(name)];
                for attr in attrs {
                    v.extend(attr.into_parts())
                }
                v.push(Part::OpeningTagEnd);
                v
            }
            Self::Closing { name } => vec![
                Part::ClosingTagStart,
                Part::ClosingTagName(name),
                Part::ClosingTagEnd,
            ],
        }
    }
}

impl Node {
    /// Converts an HTML node into smaller parts.
    pub(crate) fn into_parts(self) -> Vec<Part> {
        match self {
            Node::Doctype(_) => vec![Part::Doctype],
            Node::Tag(tag) => tag.into_parts(),
            Node::Value(value) => vec![Part::Value(value)],
        }
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::LitStr(lit_str) => lit_str.to_tokens(tokens),
            Value::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}
