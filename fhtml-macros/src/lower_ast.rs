use crate::ast;

/// A small, fine grained representation of an HTML node.
///
/// This is useful for token processing, formatting, and more, where a complete
/// HTML node representation would otherwise be limiting and insufficient.
#[derive(Debug, PartialEq)]
pub(crate) enum Part {
    Doctype,

    // Opening tag
    OpeningTagStart,
    OpeningTagName(ast::DashIdent),
    OpeningTagEnd,

    // Closing tag
    ClosingTagStart,
    ClosingTagName(ast::DashIdent),
    ClosingTagEnd,

    // Attribute
    AttrSpace,
    AttrName(ast::DashIdent),
    AttrEqSep,
    AttrValueStartQuote,
    AttrValue(ast::Value),
    AttrValueEndQuote,

    // Stray value
    Value(ast::Value),
}

impl ast::Attr {
    /// Converts an HTML attribute into smaller parts.
    pub(crate) fn into_parts(self) -> [Part; 6] {
        [
            Part::AttrSpace,
            Part::AttrName(self.name),
            Part::AttrEqSep,
            Part::AttrValueStartQuote,
            Part::AttrValue(self.value),
            Part::AttrValueEndQuote,
        ]
    }
}

impl ast::Tag {
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

impl ast::Node {
    /// Converts an HTML node into smaller parts.
    pub(crate) fn into_parts(self) -> Vec<Part> {
        match self {
            ast::Node::Doctype(_) => vec![Part::Doctype],
            ast::Node::Tag(tag) => tag.into_parts(),
            ast::Node::Value(value) => vec![Part::Value(value)],
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    use super::*;

    macro_rules! dash_ident {
        ($($arg:tt)*) => {
            syn::parse2::<crate::ast::DashIdent>(
                quote::quote! { $($arg)* }
            ).unwrap()
        }
    }

    #[test]
    fn attrs() {
        assert_eq!(
            ast::Attr {
                name: dash_ident!(foo),
                value: ast::Value::LitStr(syn::LitStr::new(
                    "foo",
                    Span::mixed_site()
                )),
            }
            .into_parts(),
            [
                Part::AttrSpace,
                Part::AttrName(dash_ident!(foo)),
                Part::AttrEqSep,
                Part::AttrValueStartQuote,
                Part::AttrValue(ast::Value::LitStr(syn::LitStr::new(
                    "foo",
                    Span::mixed_site()
                ))),
                Part::AttrValueEndQuote,
            ]
        )
    }

    #[test]
    fn opening_tag() {
        assert_eq!(
            ast::Tag::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing: false,
            }
            .into_parts(),
            [
                Part::OpeningTagStart,
                Part::OpeningTagName(dash_ident!(foo)),
                Part::OpeningTagEnd,
            ]
        );
    }

    #[test]
    fn closing_tag() {
        assert_eq!(
            ast::Tag::Closing {
                name: dash_ident!(foo),
            }
            .into_parts(),
            [
                Part::ClosingTagStart,
                Part::ClosingTagName(dash_ident!(foo)),
                Part::ClosingTagEnd,
            ]
        );
    }
}
