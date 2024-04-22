use std::fmt;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::ext::IdentExt as _;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(html);
}

struct DashIdent(Punctuated<syn::Ident, syn::Token![-]>);

impl fmt::Display for DashIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pair in self.0.pairs() {
            std::write!(f, "{}", pair.value())?;
            if pair.punct().is_some() {
                std::write!(f, "-")?;
            }
        }
        Ok(())
    }
}

impl Parse for DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(input, syn::Ident::parse_any)?))
    }
}

struct Doctype;

impl Parse for Doctype {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![!]>()?;
        input.parse::<kw::DOCTYPE>()?;
        input.parse::<kw::html>()?;
        input.parse::<syn::Token![>]>()?;

        Ok(Self)
    }
}

impl fmt::Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::write!(f, "<!DOCTYPE html>")
    }
}

#[derive(Clone)]
enum Value {
    /// A text literal, such as "Hello, World!"
    Text(syn::LitStr),

    /// A 'braced' value, such as {1 + 1}
    Braced(syn::Expr),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::write!(f, "{{}}")
    }
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Self::Text(input.parse()?))
        } else {
            let expr;
            syn::braced!(expr in input);
            Ok(Self::Braced(expr.parse()?))
        }
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Text(lit) => lit.to_tokens(tokens),
            Value::Braced(expr) => expr.to_tokens(tokens),
        }
    }
}

struct Attribute {
    name: DashIdent,
    value: Value,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { name, value })
    }
}

enum Tag {
    Start {
        name: DashIdent,
        attributes: Vec<Attribute>,
        self_closing: bool,
    },
    End {
        name: DashIdent,
    },
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Start {
                name,
                attributes,
                self_closing,
            } => {
                std::write!(f, "<{}", name)?;

                for attr in attributes {
                    std::write!(f, " {}=\"{}\"", attr.name, attr.value)?;
                }

                if *self_closing {
                    std::write!(f, "/")?;
                }

                std::write!(f, ">")
            }
            Tag::End { name } => {
                std::write!(f, "</{}>", name)
            }
        }
    }
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name;
        let mut attributes = vec![];
        let mut self_closing = false;

        // The first token should always be `<`
        input.parse::<syn::Token![<]>()?;

        // If the second token is `/`, treat it as a closing tag
        if input.peek(syn::Token![/]) {
            input.parse::<syn::Token![/]>()?;
            name = input.parse()?;
            input.parse::<syn::Token![>]>()?;

            return Ok(Self::End { name });
        }

        name = input.parse()?;

        while !(input.peek(syn::Token![>])
            || (input.peek(syn::Token![/]) && input.peek2(syn::Token![>])))
        {
            attributes.push(input.parse()?);
        }

        // If the second to last token is `/`, treat it as a self-closing tag
        if input.peek(syn::Token![/]) {
            input.parse::<syn::Token![/]>()?;
            self_closing = true;
        }

        input.parse::<syn::Token![>]>()?;

        Ok(Self::Start {
            name,
            attributes,
            self_closing,
        })
    }
}

enum Segment {
    /// An html doctype `<!DOCTYPE html>`
    Doctype(Doctype),

    /// An html tag
    Tag(Tag),

    /// An html value
    Value(Value),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Doctype(doctype) => doctype.fmt(f),
            Segment::Tag(tag) => tag.fmt(f),
            Segment::Value(value) => value.fmt(f),
        }
    }
}

impl Parse for Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![<])
            && input.peek2(syn::Token![!])
            && input.peek3(kw::DOCTYPE)
        {
            Ok(Segment::Doctype(input.parse()?))
        } else if input.peek(syn::Token![<]) {
            Ok(Segment::Tag(input.parse()?))
        } else if input.peek(syn::LitStr) || input.peek(syn::token::Brace) {
            Ok(Segment::Value(input.parse()?))
        } else {
            Err(input.error("unexpected token"))
        }
    }
}

struct Template {
    // Segments
    segments: Vec<Segment>,

    /// Any values passed to the template
    values: Vec<Value>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            segment.fmt(f)?;
        }
        Ok(())
    }
}

impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = vec![];
        let mut values = vec![];

        while !input.is_empty() {
            let segment = input.parse::<Segment>()?;

            if let Segment::Tag(Tag::Start { attributes, .. }) = &segment {
                for attr in attributes {
                    values.push(attr.value.clone());
                }
            }

            if let Segment::Value(value) = &segment {
                values.push(value.clone());
            }

            segments.push(segment);
        }

        Ok(Self { segments, values })
    }
}

struct WriteInput {
    formatter: syn::Expr,
    template: Template,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let formatter = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let template = input.parse()?;

        Ok(Self {
            formatter,
            template,
        })
    }
}

/// Writes HTML content with embedded Rust expressions into a buffer.
///
/// This macro simplifies writing HTML content by allowing Rust expressions to
/// be embedded directly within HTML tags. It uses `std::write!` internally,
/// thereby supporting all formatting capabilities and constraints of
/// `std::write!`.
///
/// ## Syntax
///
/// The `fhtml::write!` macro syntax closely resembles regular HTML, but allows
/// Rust expressions to be inserted within curly braces `{}`. These expressions
/// are evaluated and their results are inserted into the HTML output at the
/// corresponding location.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// use std::fmt::Write;
///
/// let mut buffer = String::new();
/// fhtml_macros::write!(buffer, <div>{1 + 1}</div>);
/// // This expands to:
/// // std::write!(buffer, "<div>{}</div>", 1 + 1);
///
/// assert_eq!(buffer, "<div>2</div>");
/// ```
///
/// This example demonstrates how to use `fhtml::write!` within an
/// implementation of `std::fmt::Display`:
///
/// ```rust
/// use std::fmt;
/// use std::fmt::Write;
///
/// struct Heading {
///     title: String,
/// }
///
/// impl fmt::Display for Heading {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         fhtml_macros::write!(f, <h1>{self.title}</h1>)
///     }
/// }
///
/// let heading = Heading { title: "Hello, World!".into() }.to_string();
/// assert_eq!(heading, "<h1>Hello, World!</h1>");
/// ```
///
/// ## Notes
///
/// - The macro outputs HTML directly into the provided buffer.
/// - Similar to `std::write!`, errors during writing are handled by returning a
///   `Result` indicating either success (`Ok`) or an error (`Err`).
/// - Ensure proper escaping or sanitization of user-generated content to
///   prevent injection attacks when outputting HTML.
#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    let WriteInput {
        formatter,
        template,
    } = syn::parse_macro_input!(input as WriteInput);

    let values = &template.values;
    let template = template.to_string();

    let output = quote! {
        ::std::write!(#formatter, #template, #(#values),*)
    };

    output.into()
}
