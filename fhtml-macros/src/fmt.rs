use std::fmt::{self, Write};

use crate::ast;

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

impl fmt::Display for ast::Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ast::Value::LitStr(lit_str) => lit_str.value().fmt(f),
            ast::Value::Expr(syn::Expr::Lit(syn::ExprLit { lit, .. })) => {
                match lit {
                    syn::Lit::Str(lit_str) => lit_str.value().fmt(f),
                    syn::Lit::Byte(lit_byte) => lit_byte.value().fmt(f),
                    syn::Lit::Char(lit_char) => lit_char.value().fmt(f),
                    syn::Lit::Int(lit_int) => lit_int.fmt(f),
                    syn::Lit::Float(lit_float) => lit_float.fmt(f),
                    syn::Lit::Bool(lit_bool) => lit_bool.value().fmt(f),
                    _ => Err(fmt::Error),
                }
            }
            _ => Err(fmt::Error),
        }
    }
}

impl fmt::Display for ast::Part {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctype => f.write_str("<!DOCTYPE html>"),

            // Opening tags starts with '<'
            Self::OpeningTagStart => f.write_char('<'),
            // Opening tag name
            Self::OpeningTagName(name) => name.fmt(f),
            // Opening tags ends with '>'
            Self::OpeningTagEnd => f.write_char('>'),

            // Closing tags starts with "</"
            Self::ClosingTagStart => f.write_str("</"),
            // Closing tag name
            Self::ClosingTagName(name) => name.fmt(f),
            // Closing tag ends with '>'
            Self::ClosingTagEnd => f.write_char('>'),

            // An attribute is always preceeded by a space
            Self::AttrSpace => f.write_char(' '),
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
