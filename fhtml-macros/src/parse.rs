use syn::parse::{Parse, ParseStream};

use crate::{ConcatInput, FormatArgsInput};

impl Parse for FormatArgsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            nodes: fhtml_parser::parse(input)?,
        })
    }
}

impl Parse for ConcatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            nodes: fhtml_parser::parse(input)?,
        })
    }
}

fn rm_whitespace(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut last_end = 0;
    for (start, part) in input.match_indices(' ') {
        result.push_str(unsafe { input.get_unchecked(last_end..start) });
        last_end = start + part.len();
    }
    result.push_str(unsafe { input.get_unchecked(last_end..input.len()) });
    result
}
