use std::fmt;

use quote::ToTokens;
use tiny_rsx as rsx;

use crate::ast;

fn escape_braces(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '{' => {
                output.push_str("{{");
            }
            '}' => {
                output.push_str("}}");
            }
            _ => output.push(c),
        }
    }

    output
}

impl fmt::Display for ast::Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ast::Tag::Start {
                name,
                attributes,
                self_closing,
            } => {
                write!(f, "<{name}")?;
                for ast::Attr { name, value } in attributes {
                    write!(f, " {name}=\"{value}\"")?;
                }
                if *self_closing {
                    write!(f, " /")?;
                }
                write!(f, ">")
            }
            ast::Tag::End { name } => write!(f, "</{name}>"),
        }
    }
}

impl ast::Value {
    /// Optimize certain values to be inlined instead of placeholders
    pub fn inlined(&self) -> Option<String> {
        match self {
            // Text literals are always inlined
            ast::Value::Text(text) => Some(text.value()),
            // Braced values are inlined in some cases
            ast::Value::Braced(braced) => {
                if braced.specs.is_some() || braced.escape_flag {
                    return None;
                }

                rsx::Value::Braced(braced.value.clone().into_token_stream())
                    .inlined()
            }
        }
    }
}

impl fmt::Display for ast::Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.inlined() {
            return f.write_str(&escape_braces(&value));
        }

        match self {
            Self::Text(_) => f.write_str("{}"),
            Self::Braced(braced) => {
                write!(f, "{{")?;
                if let Some(specs) = &braced.specs {
                    write!(f, ":")?;
                    specs.to_string().replace(' ', "").fmt(f)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for ast::Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ast::Segment::Doctype(doctype) => doctype.fmt(f),
            ast::Segment::Tag(tag) => tag.fmt(f),
            ast::Segment::Value(value) => value.fmt(f),
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
