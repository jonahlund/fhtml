use std::{
    borrow::Cow,
    fmt::{self, Write},
};

use fhtml_core::Render;

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
            ast::Value::LitStr(lit_str) => lit_str.value().render_to(f),
            ast::Value::Expr(expr) => {
                let mut current = Cow::Borrowed(expr);
                loop {
                    match current.as_ref() {
                        syn::Expr::Lit(syn::ExprLit { lit, .. }) => {
                            break match lit {
                                syn::Lit::Str(lit_str) => {
                                    lit_str.value().render_to(f)
                                }
                                syn::Lit::Byte(lit_byte) => {
                                    lit_byte.value().render_to(f)
                                }
                                syn::Lit::Char(lit_char) => {
                                    lit_char.value().render_to(f)
                                }
                                syn::Lit::Int(lit_int) => lit_int.fmt(f),
                                syn::Lit::Float(lit_float) => lit_float.fmt(f),
                                syn::Lit::Bool(lit_bool) => {
                                    lit_bool.value().render_to(f)
                                }
                                _ => Err(fmt::Error),
                            }
                        }
                        syn::Expr::Const(expr_const) => {
                            current = Cow::Owned(
                                expr_in_const(expr_const)
                                    .ok_or(fmt::Error)?
                                    .clone(),
                            );
                        }
                        syn::Expr::Paren(expr_paren) => {
                            current = Cow::Owned(*expr_paren.expr.clone());
                        }
                        syn::Expr::Reference(expr_reference) => {
                            current = Cow::Owned(*expr_reference.expr.clone());
                        }
                        _ => break Err(fmt::Error),
                    }
                }
            }
        }
    }
}

impl fmt::Display for ast::Part {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctype => f.write_str("<!DOCTYPE html>"),

            // Opening tag
            Self::OpeningTagStart => f.write_char('<'),
            Self::OpeningTagName(name) => name.fmt(f),
            Self::OpeningTagEnd => f.write_char('>'),

            // Closing tag
            Self::ClosingTagStart => f.write_str("</"),
            Self::ClosingTagName(name) => name.fmt(f),
            Self::ClosingTagEnd => f.write_char('>'),

            // Attribute
            Self::AttrSpace => f.write_char(' '),
            Self::AttrName(name) => name.fmt(f),
            Self::AttrEqSep => f.write_char('='),
            Self::AttrValueOpeningQuote => f.write_char('"'),
            Self::AttrValue(value) => value.fmt(f),
            Self::AttrValueClosingQuote => f.write_char('"'),

            // Stray value
            Self::Value(value) => value.fmt(f),
        }
    }
}

fn expr_in_const(
    syn::ExprConst { block, .. }: &syn::ExprConst,
) -> Option<&syn::Expr> {
    match block.stmts.last().take()? {
        syn::Stmt::Expr(expr, None) => Some(expr),
        _ => None,
    }
}
