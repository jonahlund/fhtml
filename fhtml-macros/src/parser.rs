use crate::entity;
use syn::parse::{Parse, ParseStream};

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
        let ident: syn::Ident = input.parse()?;
        value.push_str(&ident.to_string());

        while input.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            let ident: syn::Ident = input.parse()?;
            value.push('-');
            value.push_str(&ident.to_string());
        }

        Ok(entity::DashIdent { value })
    }
}

impl Parse for entity::Html {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut parts = Vec::new();
        let mut values = Vec::new();

        while !input.is_empty() {
            if input.peek(syn::Token![<]) && input.peek2(syn::Token![/]) {
                input.parse::<syn::Token![<]>()?;
                input.parse::<syn::Token![/]>()?;

                let name = input.parse()?;

                input.parse::<syn::Token![>]>()?;

                parts.push(entity::Part::ClosingTag { name });

                continue;
            }

            if input.peek(syn::Token![<]) {
                input.parse::<syn::Token![<]>()?;

                let name = input.parse()?;
                let mut attributes = Vec::new();

                while input.peek(syn::Ident) {
                    let name = input.parse()?;
                    input.parse::<syn::Token![=]>()?;

                    attributes.push(entity::Attribute { name });
                    values.push(input.parse()?);
                }

                if input.peek(syn::Token![/]) {
                    input.parse::<syn::Token![/]>()?;
                    input.parse::<syn::Token![>]>()?;
                    parts.push(entity::Part::SelfClosingTag { name, attributes });
                } else {
                    input.parse::<syn::Token![>]>()?;
                    parts.push(entity::Part::OpeningTag { name, attributes });
                }

                continue;
            }

            if input.peek(syn::LitStr) {
                values.push(input.parse()?);
                parts.push(entity::Part::Value);
                continue;
            }

            if input.peek(syn::token::Brace) {
                values.push(input.parse()?);
                parts.push(entity::Part::Value);
                continue;
            }

            break;
        }

        Ok(entity::Html { parts, values })
    }
}
