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
        unreachable!()
    }
}

impl fmt::Display for ast::PlaceholderValue {
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

#[rustfmt::skip]
impl<Value: fmt::Display> fmt::Display for lower_ast::AstPart<Value> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctype              => f.write_str("<!DOCTYPE html>"),
            Self::OpeningTagStart      => f.write_char('<'),
            Self::OpeningTagName(name) => name.fmt(f),
            Self::OpeningTagEnd        => f.write_char('>'),
            Self::ClosingTagStart      => f.write_str("</"),
            Self::ClosingTagName(name) => name.fmt(f),
            Self::ClosingTagEnd        => f.write_char('>'),
            Self::AttrName(name)       => write!(f, " {name}="),
            Self::AttrValueStart       => f.write_char('"'),
            Self::AttrValue(value)     => write!(f, "{value}"),
            Self::AttrValueEnd         => f.write_char('"'),
            Self::Value(value)         => value.fmt(f),
        }
    }
}
