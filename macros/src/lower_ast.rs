use crate::ast;

/// A small, fine grained representation of an HTML node.
///
/// This is useful for token processing, formatting, and more, where a complete
/// HTML node representation would otherwise be limiting and insufficient.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum NodeToken<V> {
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
    AttrStartSpace,
    AttrName(ast::DashIdent),
    AttrEqSep,
    AttrValueStartQuote,
    AttrValue(V),
    AttrValueEndQuote,

    // Stray value
    Value(V),
}

impl<V> ast::Attr<V> {
    /// Converts an HTML attribute into a set of NodeTokens.
    pub(crate) fn into_node_tokens(self) -> [NodeToken<V>; 6] {
        [
            NodeToken::AttrStartSpace,
            NodeToken::AttrName(self.name),
            NodeToken::AttrEqSep,
            NodeToken::AttrValueStartQuote,
            NodeToken::AttrValue(self.value),
            NodeToken::AttrValueEndQuote,
        ]
    }
}

impl<V> ast::Tag<V> {
    /// Converts an HTML tag into a set of NodeTokens.
    pub(crate) fn into_node_tokens(self) -> Vec<NodeToken<V>> {
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

impl<V> ast::Node<V> {
    /// Converts an HTML node into a set of NodeTokens.
    pub(crate) fn into_node_tokens(self) -> Vec<NodeToken<V>> {
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
    fn attrs() {
        assert_eq!(
            ast::Attr {
                name: dash_ident!(foo),
                eq_sep: syn::Token![=]([Span::mixed_site()]),
                value: ast::LitValue::LitStr(syn::LitStr::new(
                    "foo",
                    Span::mixed_site()
                )),
            }
            .into_node_tokens(),
            [
                NodeToken::AttrStartSpace,
                NodeToken::AttrName(dash_ident!(foo)),
                NodeToken::AttrEqSep,
                NodeToken::AttrValueStartQuote,
                NodeToken::AttrValue(ast::LitValue::LitStr(syn::LitStr::new(
                    "foo",
                    Span::mixed_site()
                ))),
                NodeToken::AttrValueEndQuote,
            ]
        )
    }

    #[test]
    fn opening_tag() {
        assert_eq!(
            ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing_slash: None
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
