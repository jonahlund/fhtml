use std::fmt::{self, Write};

use crate::{ast, lower_ast};

impl fmt::Display for ast::DashIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pair in self.0.pairs() {
            pair.value().fmt(f)?;
            if pair.punct().is_some() {
                f.write_char('-')?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for ast::LitValue {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Display for ast::ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LitStr(_) => f.write_str("{}"),
            Self::Expr { specs, .. } => {
                f.write_char('{')?;
                if let Some(specs) = specs {
                    f.write_char(':')?;
                    specs.to_string().replace(' ', "").fmt(f)?;
                }
                f.write_char('}')
            }
        }
    }
}

impl<V: fmt::Display> fmt::Display for lower_ast::NodeToken<V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctype => f.write_str("<!DOCTYPE html>"),

            // Opening tags start with '<'
            Self::OpeningTagStart => f.write_char('<'),
            // Opening tag name
            Self::OpeningTagName(name) => name.fmt(f),
            // Opening tags end with '>'
            //
            // We do not include a forward slash '/' for self-closing
            // tags.
            Self::OpeningTagEnd => f.write_char('>'),

            // Closing tags start with "</"
            Self::ClosingTagStart => f.write_str("</"),
            // Closing tag name
            Self::ClosingTagName(name) => name.fmt(f),
            // Closing tag ends with '>'
            Self::ClosingTagEnd => f.write_char('>'),

            // An attribute is always preceeded by a space
            Self::AttrStartSpace => f.write_char(' '),
            // Attribute name
            Self::AttrName(name) => name.fmt(f),
            // Equal sign, separating the attribute name from the value
            Self::AttrEqSep => f.write_char('='),
            // Starting quote
            Self::AttrValueStartQuote => f.write_char('"'),
            // Attribute value
            Self::AttrValue(value) => value.fmt(f),
            // Ending quote
            Self::AttrValueEndQuote => f.write_char('"'),

            // Stray value
            Self::Value(value) => value.fmt(f),
        }
    }
}
