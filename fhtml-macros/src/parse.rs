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

        let self_closing = input.peek(syn::Token![/]);
        if self_closing {
            input.parse::<syn::Token![/]>()?;
        }

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
        if input.peek(syn::LitStr) {
            Ok(Self::Text(input.parse()?))
        } else {
            let content;
            syn::braced!(content in input);

            let value = content.parse()?;
            let mut specs = None;
            let mut escape = false;

            if content.peek(syn::Token![:]) {
                content.parse::<syn::Token![:]>()?;
                if content.peek(syn::Token![!]) {
                    content.parse::<syn::Token![!]>()?;
                    escape = true;
                }
                if !content.is_empty() {
                    specs = Some(content.parse()?);
                }
            }

            Ok(Self::Braced(ast::BracedValue {
                value,
                specs,
                escape,
            }))
        }
    }
}

impl Parse for ast::Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![<])
            && input.peek2(syn::Token![!])
            && (input.peek3(kw::DOCTYPE) || input.peek3(kw::doctype))
        {
            Ok(Self::Doctype(input.parse()?))
        } else if input.peek(syn::Token![<]) {
            Ok(Self::Tag(input.parse()?))
        } else if input.peek(syn::LitStr) || input.peek(syn::token::Brace) {
            Ok(Self::Value(input.parse()?))
        } else {
            Err(input.error("unexpected segment"))
        }
    }
}

impl Parse for ast::Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = vec![];
        let mut values = vec![];

        while !input.is_empty() {
            let segment = input.parse::<ast::Segment>()?;

            match &segment {
                ast::Segment::Tag(ast::Tag::Start { attributes, .. }) => {
                    for attr in attributes {
                        if attr.value.inlined().is_none() {
                            values.push(attr.value.clone());
                        }
                    }
                }
                ast::Segment::Value(value) => {
                    if value.inlined().is_none() {
                        values.push(value.clone());
                    }
                }
                _ => {}
            };

            segments.push(segment);
        }

        Ok(Self { segments, values })
    }
}
