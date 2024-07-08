use core::fmt;
use std::vec::IntoIter;

use syn::parse::Parse;

use crate::ast;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum AstPart<Value: fmt::Display> {
    Doctype,
    OpeningTagStart,
    OpeningTagName(ast::DashIdent),
    OpeningTagEnd,
    ClosingTagStart,
    ClosingTagName(ast::DashIdent),
    ClosingTagEnd,
    AttrName(ast::DashIdent),
    AttrValueStart,
    AttrValue(Value),
    AttrValueEnd,
    Value(Value),
}

impl<Value: Parse + fmt::Display> IntoIterator for ast::Attr<Value> {
    type IntoIter = core::array::IntoIter<AstPart<Value>, 4>;
    type Item = AstPart<Value>;

    fn into_iter(self) -> Self::IntoIter {
        [
            AstPart::AttrName(self.name),
            AstPart::AttrValueStart,
            AstPart::AttrValue(self.value),
            AstPart::AttrValueEnd,
        ]
        .into_iter()
    }
}

impl<Value: Parse + fmt::Display> IntoIterator for ast::Tag<Value> {
    type IntoIter = IntoIter<AstPart<Value>>;
    type Item = AstPart<Value>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Opening { name, attrs, .. } => {
                let mut v = vec![
                    AstPart::OpeningTagStart,
                    AstPart::OpeningTagName(name),
                ];
                for attr in attrs {
                    v.extend(attr.into_iter())
                }
                v.push(AstPart::OpeningTagEnd);
                v
            }
            Self::Closing { name } => vec![
                AstPart::ClosingTagStart,
                AstPart::ClosingTagName(name),
                AstPart::ClosingTagEnd,
            ],
        }
        .into_iter()
    }
}

impl<Value: Parse + fmt::Display> IntoIterator for ast::Node<Value> {
    type IntoIter = IntoIter<AstPart<Value>>;
    type Item = AstPart<Value>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ast::Node::Doctype(_) => vec![AstPart::Doctype].into_iter(),
            ast::Node::Tag(tag) => tag.into_iter(),
            ast::Node::Value(value) => vec![AstPart::Value(value)].into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! dash_ident {
        ($($arg:tt)*) => {
            syn::parse2::<crate::ast::DashIdent>(
                quote::quote! { $($arg)* }
            ).unwrap()
        }
    }

    #[test]
    fn opening_tags() {
        assert_eq!(
            ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing: false
            }
            .into_iter()
            .collect::<Vec<_>>(),
            vec![
                AstPart::OpeningTagStart,
                AstPart::OpeningTagName(dash_ident!(foo)),
                AstPart::OpeningTagEnd,
            ]
        );
    }
}
