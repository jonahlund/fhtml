use quote::ToTokens;
use syn::punctuated::Punctuated;

/// Represents an identifier separated by dashes, e.g., `foo-bar-baz`.
pub struct DashIdent(pub Punctuated<syn::Ident, syn::Token![-]>);

/// Represents the DOCTYPE declaration in HTML, e.g., `<!DOCTYPE html>`.
pub struct Doctype;

#[derive(Clone, Copy)]
pub enum FormatSpecifier {
    Display,
    Debug,
    DebugLowerHex,
    DebugUpperHex,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
}

#[derive(Clone)]
pub enum Value {
    /// A text literal, such as "Hello, World!"
    Text(syn::LitStr),

    /// A 'braced' value, such as `{1 + 1}`.
    Braced(syn::Expr, FormatSpecifier),
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Text(lit) => lit.to_tokens(tokens),
            Value::Braced(expr, _) => expr.to_tokens(tokens),
        }
    }
}

/// Represents an HTML attribute, consisting of a name-value pair.
pub struct Attribute {
    pub name: DashIdent,
    pub value: Value,
}

/// Represents an HTML tag, which can be either a start tag with attributes and
/// optional self-closing flag, or an end tag.
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

pub enum Segment {
    /// Represents an HTML `<!DOCTYPE html>` declaration.
    Doctype(Doctype),

    /// Represents an HTML tag, which may be a start tag, an end tag or a
    /// self-closing tag.
    Tag(Tag),

    /// Represents text or interpolated values within an HTML document.
    Value(Value),
}

pub struct Template {
    // Contains the segments of the HTML template.
    pub segments: Vec<Segment>,

    /// Stores values that are interpolated into the template.
    pub values: Vec<Value>,
}
