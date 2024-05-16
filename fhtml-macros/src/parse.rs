use syn::ext::IdentExt as _;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use crate::html;

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(html);
    syn::custom_keyword!(x);
    syn::custom_keyword!(X);
    syn::custom_keyword!(o);
    syn::custom_keyword!(p);
    syn::custom_keyword!(b);
    syn::custom_keyword!(e);
    syn::custom_keyword!(E);
}

impl Parse for html::DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse a non-empty sequence of identifiers separated by dashes.
        Ok(Self(Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(input, syn::Ident::parse_any)?))
    }
}

impl Parse for html::Doctype {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![!]>()?;
        input.parse::<kw::DOCTYPE>()?;
        input.parse::<kw::html>()?;
        input.parse::<syn::Token![>]>()?;

        Ok(Self)
    }
}

impl Parse for html::FormatSpecifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // No formatting specifier is Display
        if !input.peek(syn::Token![:]) {
            return Ok(Self::Display);
        }

        input.parse::<syn::Token![:]>()?;

        // Debug
        if input.peek(syn::Token![?]) {
            input.parse::<syn::Token![?]>()?;
            return Ok(Self::Debug);
        }

        // Debug with lower-case hexadecimal integers
        if input.peek(kw::x) {
            input.parse::<kw::x>()?;
            if input.peek(syn::Token![?]) {
                input.parse::<syn::Token![?]>()?;
                return Ok(Self::DebugLowerHex);
            }
            return Ok(Self::LowerHex);
        }

        // Debug with upper-case hexadecimal integers
        if input.peek(kw::X) {
            input.parse::<kw::X>()?;
            if input.peek(syn::Token![?]) {
                input.parse::<syn::Token![?]>()?;
                return Ok(Self::DebugUpperHex);
            }
            return Ok(Self::UpperHex);
        }

        // Octal
        if input.peek(kw::o) {
            input.parse::<kw::o>()?;
            return Ok(Self::Octal);
        }

        // Pointer
        if input.peek(kw::p) {
            input.parse::<kw::p>()?;
            return Ok(Self::Pointer);
        }

        // Binary
        if input.peek(kw::b) {
            input.parse::<kw::b>()?;
            return Ok(Self::Binary);
        }

        // LowerExp
        if input.peek(kw::e) {
            input.parse::<kw::e>()?;
            return Ok(Self::LowerExp);
        }

        // UpperExp
        if input.peek(kw::E) {
            input.parse::<kw::E>()?;
            return Ok(Self::UpperExp);
        }

        Err(input.error("invalid formatting specifier"))
    }
}

impl Parse for html::Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Self::Text(input.parse()?))
        } else {
            let expr;
            syn::braced!(expr in input);
            Ok(Self::Braced(expr.parse()?, expr.parse()?))
        }
    }
}

impl Parse for html::Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { name, value })
    }
}

impl Parse for html::Tag {
    /// Parses an HTML tag, which could be a start tag with attributes and an
    /// optional self-closing indicator, or an end tag.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the opening angle bracket
        input.parse::<syn::Token![<]>()?;

        // Check if it's an end tag
        let forward_slash = input.peek(syn::Token![/]);
        if forward_slash {
            input.parse::<syn::Token![/]>()?;

            // Parse the tag name
            let name: html::DashIdent = input.parse()?;

            // Parse the closing '>'
            input.parse::<syn::Token![>]>()?;

            // Return the end tag variant
            return Ok(html::Tag::End { name });
        }

        // Parse the tag name for start tag
        let name: html::DashIdent = input.parse()?;

        // Parse attributes if any
        let mut attributes = Vec::new();
        while !(input.peek(syn::Token![>])
            || (input.peek(syn::Token![/]) && input.peek2(syn::Token![>])))
        {
            attributes.push(input.parse()?);
        }

        // Check for self-closing tag
        let self_closing = input.peek(syn::Token![/]);
        if self_closing {
            // Consume '/>'
            input.parse::<syn::Token![/]>()?;
            input.parse::<syn::Token![>]>()?;
        } else {
            // Consume '>'
            input.parse::<syn::Token![>]>()?;
        }

        // Return the start tag variant
        Ok(html::Tag::Start {
            name,
            attributes,
            self_closing,
        })
    }
}

impl Parse for html::Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![<])
            && input.peek2(syn::Token![!])
            && input.peek3(kw::DOCTYPE)
        {
            Ok(html::Segment::Doctype(input.parse()?))
        } else if input.peek(syn::Token![<]) {
            Ok(html::Segment::Tag(input.parse()?))
        } else if input.peek(syn::LitStr) || input.peek(syn::token::Brace) {
            Ok(html::Segment::Value(input.parse()?))
        } else {
            Err(input.error("unexpected token"))
        }
    }
}

impl Parse for html::Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = vec![];
        let mut values = vec![];

        while !input.is_empty() {
            let segment = input.parse::<html::Segment>()?;

            if let html::Segment::Tag(html::Tag::Start { attributes, .. }) =
                &segment
            {
                for attr in attributes {
                    values.push(attr.value.clone());
                }
            }

            if let html::Segment::Value(value) = &segment {
                values.push(value.clone());
            }

            segments.push(segment);
        }

        Ok(Self { segments, values })
    }
}
