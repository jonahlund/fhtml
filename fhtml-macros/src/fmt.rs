use std::fmt::{self, Write};

use crate::html::{
    Attribute, DashIdent, Doctype, Segment, Tag, Template, Value,
};

impl fmt::Display for DashIdent {
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

impl fmt::Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<!DOCTYPE html>")
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start {
                name,
                attributes,
                self_closing,
            } => {
                write!(f, "<{name}")?;
                for Attribute { name, value } in attributes {
                    write!(f, " {name}=\"{value}\"")?;
                }
                if *self_closing {
                    write!(f, " /")?;
                }
                write!(f, ">")
            }
            Self::End { name } => write!(f, "</{name}>"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(_) => f.write_str("{}"),
            Self::Braced { params, .. } => {
                write!(f, "{{")?;
                if let Some(params) = params {
                    write!(f, ":")?;
                    params.to_string().replace(' ', "").fmt(f)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctype(doctype) => doctype.fmt(f),
            Self::Tag(tag) => tag.fmt(f),
            Self::Value(value) => value.fmt(f),
        }
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            segment.fmt(f)?;
        }
        Ok(())
    }
}
