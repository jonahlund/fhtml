use syn::parse::{Parse, ParseStream};
use tiny_rsx as rsx;

use crate::ast;

impl Parse for ast::BracedValueWithSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        let mut specs = None;
        let mut escape_flag = false;

        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            if input.peek(syn::Token![!]) {
                input.parse::<syn::Token![!]>()?;
                escape_flag = true;
            }
            if !input.is_empty() {
                specs = Some(input.parse()?);
            }
        }

        Ok(Self {
            value,
            specs,
            escape_flag,
        })
    }
}

impl Parse for ast::Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = vec![];
        let mut values = vec![];

        while !input.is_empty() {
            let segment = input.parse::<rsx::Segment>()?;

            match &segment {
                rsx::Segment::Tag(rsx::Tag::Start { attributes, .. }) => {
                    for attr in attributes {
                        if attr.value.inlined().is_none() {
                            values.push(attr.value.clone().try_into()?);
                        }
                    }
                }
                rsx::Segment::Value(value) => {
                    if value.inlined().is_none() {
                        values.push(value.clone().try_into()?);
                    }
                }
                _ => {}
            };

            segments.push(segment.try_into()?);
        }

        Ok(Self { segments, values })
    }
}
