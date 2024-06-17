use syn::ext::IdentExt as _;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use crate::ast;

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(doctype);
    syn::custom_keyword!(html);
}

impl Parse for ast::DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse a non-empty sequence of identifiers separated by dashes.
        Ok(Self(
            Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(
                input,
                syn::Ident::parse_any,
            )?
        ))
    }
}

impl Parse for ast::Doctype {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![!]>()?;
        if input.peek(kw::doctype) {
            input.parse::<kw::doctype>()?;
        } else {
            input.parse::<kw::DOCTYPE>()?;
        }
        input.parse::<kw::html>()?;
        input.parse::<syn::Token![>]>()?;

        Ok(Self)
    }
}

impl Parse for ast::Tag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;

        if input.peek(syn::Token![/]) {
            input.parse::<syn::Token![/]>()?;
            let name = input.parse()?;
            input.parse::<syn::Token![>]>()?;

            return Ok(Self::End { name });
        }

        let name = input.parse()?;

        let mut attributes = Vec::new();
        while !(input.peek(syn::Token![>])
            || (input.peek(syn::Token![/]) && input.peek2(syn::Token![>])))
        {
            attributes.push(input.parse()?);
        }

        let self_closing = input.parse::<Option<syn::Token![/]>>()?.is_some();
        input.parse::<syn::Token![>]>()?;

        Ok(Self::Start {
            name,
            attributes,
            self_closing,
        })
    }
}

impl Parse for ast::Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { name, value })
    }
}

impl Parse for ast::Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if lookahead.peek(syn::token::Brace) {
            let content;

            syn::braced!(content in input);

            let value = content.parse()?;
            let mut specs = None;

            if content.parse::<Option<syn::Token![:]>>()?.is_some() {
                specs = Some(content.parse()?);
            }

            Ok(Self::Braced { value, specs })
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ast::Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![<])
            && input.peek2(syn::Token![!])
            && (input.peek3(kw::DOCTYPE) || input.peek3(kw::doctype))
        {
            Ok(Self::Doctype(input.parse()?))
        } else if lookahead.peek(syn::Token![<]) {
            Ok(Self::Tag(input.parse()?))
        } else if lookahead.peek(syn::LitStr)
            || lookahead.peek(syn::token::Brace)
        {
            Ok(Self::Value(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ast::Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = vec![];
        let mut values = vec![];

        while !input.is_empty() {
            let segment = input.parse::<ast::Segment>()?;

            // Gather all values from segments
            match &segment {
                ast::Segment::Tag(ast::Tag::Start { attributes, .. }) => {
                    for attr in attributes {
                        values.push(attr.value.clone());
                    }
                }
                ast::Segment::Value(value) => {
                    values.push(value.clone());
                }
                _ => {}
            };

            segments.push(segment);
        }

        Ok(Self {
            segments,
            // Inlined values are omitted
            values: values
                .into_iter()
                .filter(|v| v.clone().into_string().is_none())
                .collect(),
        })
    }
}
