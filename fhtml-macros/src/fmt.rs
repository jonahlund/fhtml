use std::fmt::{self, Write};

use crate::html;

impl fmt::Display for html::DashIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Iterate through the punctuated identifiers and format them with
        // dashes in between.
        for pair in self.0.pairs() {
            std::write!(f, "{}", pair.value())?;
            // If there's a punctuation, it's a dash, so we append it.
            if pair.punct().is_some() {
                f.write_char('-')?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for html::Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<!DOCTYPE html>")
    }
}

impl fmt::Display for html::Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: this is not the actual value but rather just braces `{}`
        f.write_str("{}")
    }
}

impl fmt::Display for html::Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            html::Tag::Start {
                name,
                attributes,
                self_closing,
            } => {
                std::write!(f, "<{name}")?;
                for html::Attribute { name, value } in attributes {
                    std::write!(f, " {name}=\"{value}\"")?;
                }
                if *self_closing {
                    f.write_char('/')?;
                }
                f.write_char('>')
            }
            html::Tag::End { name } => std::write!(f, "</{name}>"),
        }
    }
}

impl fmt::Display for html::Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            html::Segment::Doctype(doctype) => doctype.fmt(f),
            html::Segment::Tag(tag) => tag.fmt(f),
            html::Segment::Value(value) => value.fmt(f),
        }
    }
}

impl fmt::Display for html::Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            segment.fmt(f)?;
        }
        Ok(())
    }
}
