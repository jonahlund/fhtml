use std::fmt::Write as _;
use std::mem;

use proc_macro2::TokenStream;
use quote::ToTokens as _;

use crate::lower_ast;

pub(crate) enum Segment {
    String(String),
    Tokens(TokenStream),
}

pub(crate) fn from_parts(parts: &[lower_ast::Part]) -> Vec<Segment> {
    let mut segs = Vec::new();
    let mut acc = String::new();

    for part in parts {
        if let lower_ast::Part::AttrValue(val) | lower_ast::Part::Value(val) =
            part
        {
            if write!(acc, "{}", val).is_err() {
                segs.push(Segment::String(mem::take(&mut acc)));
                segs.push(Segment::Tokens(val.to_token_stream()));
            }
        } else {
            let _ = write!(acc, "{}", part);
        }
    }

    if !acc.is_empty() {
        segs.push(Segment::String(acc))
    }

    segs
}
