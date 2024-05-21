use syn::ext::IdentExt as _;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use crate::html::{
    Attribute, DashIdent, Doctype, Segment, Tag, Template, Value,
};

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(doctype);
    syn::custom_keyword!(html);
}

impl Parse for DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse a non-empty sequence of identifiers separated by dashes.
        Ok(Self(Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(input, syn::Ident::parse_any)?))
    }
}

impl Parse for Doctype {
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

impl Parse for Tag {
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

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { name, value })
    }
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Self::Text(input.parse()?))
        } else {
            let content;
            syn::braced!(content in input);

            let value = content.parse()?;
            let mut params = None;
            let mut escape = false;

            if content.peek(syn::Token![:]) {
                content.parse::<syn::Token![:]>()?;
                if content.peek(syn::Token![!]) {
                    content.parse::<syn::Token![!]>()?;
                    escape = true;
                }
                if !content.is_empty() {
                    params = Some(content.parse()?);
                }
            }

            Ok(Self::Braced {
                value,
                params,
                escape,
            })
        }
    }
}

impl Parse for Segment {
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

impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = vec![];
        let mut values = vec![];

        while !input.is_empty() {
            let segment = input.parse::<Segment>()?;

            match &segment {
                Segment::Tag(Tag::Start { attributes, .. }) => {
                    for attr in attributes {
                        values.push(attr.value.clone());
                    }
                }
                Segment::Value(value) => {
                    values.push(value.clone());
                }
                _ => {}
            };

            segments.push(segment);
        }

        Ok(Self { segments, values })
    }
}
