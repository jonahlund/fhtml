use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};

use crate::entity;

impl Parse for entity::Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Self::Text(input.parse()?))
        } else {
            let expr;
            syn::braced!(expr in input);
            Ok(Self::Braced(expr.parse()?))
        }
    }
}

impl Parse for entity::DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut value = String::new();

        let mut part = input.call(syn::Ident::parse_any)?;
        value.push_str(&part.to_string());

        while input.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            part = input.call(syn::Ident::parse_any)?;
            value.push('-');
            value.push_str(&part.to_string());
        }

        Ok(entity::DashIdent { value })
    }
}

impl Parse for entity::Html {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = Vec::new();
        let mut values = Vec::new();

        while !input.is_empty() {
            if input.peek(syn::Token![<]) && input.peek2(syn::Token![/]) {
                input.parse::<syn::Token![<]>()?;
                input.parse::<syn::Token![/]>()?;

                let name = input.parse()?;

                input.parse::<syn::Token![>]>()?;

                segments.push(entity::Segment::ClosingTag { name });

                continue;
            }

            if input.peek(syn::Token![<]) {
                input.parse::<syn::Token![<]>()?;

                let name = input.parse()?;
                let mut attributes = Vec::new();

                while !input.peek(syn::Token![/]) && !input.peek(syn::Token![>])
                {
                    let name = input.parse()?;
                    input.parse::<syn::Token![=]>()?;

                    attributes.push(entity::Attribute { name });
                    values.push(input.parse()?);
                }

                if input.peek(syn::Token![/]) {
                    input.parse::<syn::Token![/]>()?;
                    input.parse::<syn::Token![>]>()?;
                    segments.push(entity::Segment::SelfClosingTag {
                        name,
                        attributes,
                    });
                } else {
                    input.parse::<syn::Token![>]>()?;
                    segments
                        .push(entity::Segment::OpeningTag { name, attributes });
                }

                continue;
            }

            if input.peek(syn::LitStr) {
                values.push(input.parse()?);
                segments.push(entity::Segment::Value);
                continue;
            }

            if input.peek(syn::token::Brace) {
                values.push(input.parse()?);
                segments.push(entity::Segment::Value);
                continue;
            }

            break;
        }

        Ok(entity::Html { segments, values })
    }
}
