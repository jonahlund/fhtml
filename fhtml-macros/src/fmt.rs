use std::fmt::{self, Write};

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
        match self {
            Self::Start {
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
            Self::End { name } => write!(f, "</{name}>"),
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
                if braced.specs.is_some() || braced.escape {
                    return None;
                }

                match &braced.value {
                    syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Str(str) => Some(str.value()),
                        syn::Lit::Byte(byte) => Some(byte.value().to_string()),
                        syn::Lit::Char(char) => Some(char.value().to_string()),
                        syn::Lit::Int(int) => {
                            Some(int.base10_digits().to_string())
                        }
                        syn::Lit::Float(float) => {
                            Some(float.base10_digits().to_string())
                        }
                        syn::Lit::Bool(bool) => Some(bool.value().to_string()),
                        _ => None,
                    },
                    _ => None,
                }
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
            Self::Text(text) => f.write_str(&text.value()),
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
