use core::fmt;

use syn::parse::Parse;

use crate::ast;

/// A small, fine grained representation of an HTML node.
///
/// This is useful for token processing, formatting, and more, where a complete
/// HTML node representation would otherwise be limiting and insufficient.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum NodeToken<Value: fmt::Display> {
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
    AttrStart,
    AttrName(ast::DashIdent),
    AttrSep,
    AttrValueStart,
    AttrValue(Value),
    AttrValueEnd,

    // Stray value
    Value(Value),
}

impl<Value: Parse + fmt::Display> ast::Attr<Value> {
    /// Converts an HTML attribute into a set of NodeTokens.
    pub(crate) fn into_node_tokens(self) -> [NodeToken<Value>; 6] {
        [
            NodeToken::AttrStart,
            NodeToken::AttrName(self.name),
            NodeToken::AttrSep,
            NodeToken::AttrValueStart,
            NodeToken::AttrValue(self.value),
            NodeToken::AttrValueEnd,
        ]
    }
}

impl<Value: Parse + fmt::Display> ast::Tag<Value> {
    /// Converts an HTML tag into a set of NodeTokens.
    pub(crate) fn into_node_tokens(self) -> Vec<NodeToken<Value>> {
        match self {
            Self::Opening { name, attrs, .. } => {
                let mut v = vec![
                    NodeToken::OpeningTagStart,
                    NodeToken::OpeningTagName(name),
                ];
                for attr in attrs {
                    v.extend(attr.into_node_tokens())
                }
                v.push(NodeToken::OpeningTagEnd);
                v
            }
            Self::Closing { name } => vec![
                NodeToken::ClosingTagStart,
                NodeToken::ClosingTagName(name),
                NodeToken::ClosingTagEnd,
            ],
        }
    }
}

impl<Value: Parse + fmt::Display> ast::Node<Value> {
    /// Converts an HTML node into a set of NodeTokens.
    pub(crate) fn into_node_tokens(self) -> Vec<NodeToken<Value>> {
        match self {
            ast::Node::Doctype(_) => vec![NodeToken::Doctype],
            ast::Node::Tag(tag) => tag.into_node_tokens(),
            ast::Node::Value(value) => vec![NodeToken::Value(value)],
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
    fn attributes() {
        assert_eq!(
            ast::Attr {
                name: dash_ident!(foo),
                value: ast::LitValue::LitStr(syn::LitStr::new(
                    "foo",
                    Span::mixed_site()
                ))
            }
            .into_node_tokens(),
            [
                NodeToken::AttrStart,
                NodeToken::AttrName(dash_ident!(foo)),
                NodeToken::AttrSep,
                NodeToken::AttrValueStart,
                NodeToken::AttrValue(ast::LitValue::LitStr(syn::LitStr::new(
                    "foo",
                    Span::mixed_site()
                ))),
                NodeToken::AttrValueEnd,
            ]
        )
    }

    #[test]
    fn opening_tag() {
        assert_eq!(
            ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing: false
            }
            .into_node_tokens(),
            [
                NodeToken::OpeningTagStart,
                NodeToken::OpeningTagName(dash_ident!(foo)),
                NodeToken::OpeningTagEnd,
            ]
        );
    }

    #[test]
    fn closing_tag() {
        assert_eq!(
            ast::Tag::<ast::LitValue>::Closing {
                name: dash_ident!(foo),
            }
            .into_node_tokens(),
            [
                NodeToken::ClosingTagStart,
                NodeToken::ClosingTagName(dash_ident!(foo)),
                NodeToken::ClosingTagEnd,
            ]
        );
    }
}
