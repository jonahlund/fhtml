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

impl fmt::Display for ast::Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<!DOCTYPE html>")
    }
}

impl fmt::Display for ast::Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ast::TagKind::Start {
                name,
                attributes,
                self_closing,
            } => {
                f.write_char('<')?;
                name.fmt(f)?;
                for ast::Attr { name, value, .. } in attributes {
                    write!(f, " {name}=\"{value}\"")?;
                }
                if *self_closing {
                    f.write_str(" /")?;
                }
                f.write_char('>')
            }
            ast::TagKind::End { name } => write!(f, "</{name}>"),
        }
    }
}

impl fmt::Display for ast::Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(_) => f.write_str("{}"),
            Self::Braced { specs, .. } => {
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

impl fmt::Display for ast::Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctype(doctype) => doctype.fmt(f),
            Self::Tag(tag) => tag.fmt(f),
            Self::Value(value) => value.fmt(f),
        }
    }
}

impl fmt::Display for ast::Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            segment.fmt(f)?;
        }
        Ok(())
    }
}
