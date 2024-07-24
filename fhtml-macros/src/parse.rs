use syn::{
    ext::IdentExt as _,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{ast, ConcatInput, FormatInput, WriteInput};

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(html);
}

impl Parse for ast::DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse a non-empty sequence of identifiers separated by dashes.
        let inner = Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(
            input,
            syn::Ident::parse_any,
        )?;

        Ok(Self(inner))
    }
}

impl Parse for ast::Doctype {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![!]>()?;
        input.parse::<kw::DOCTYPE>()?;
        input.parse::<kw::html>()?;
        input.parse::<syn::Token![>]>()?;

        Ok(Self)
    }
}

impl Parse for ast::Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            Ok(Self::LitStr(input.parse()?))
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            Ok(Self::Expr(content.parse()?))
        } else {
            Err(lookahead.error())
        }
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

impl Parse for ast::Tag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;

        if input.parse::<Option<syn::Token![/]>>()?.is_some() {
            let name = input.parse()?;
            input.parse::<syn::Token![>]>()?;

            return Ok(Self::Closing { name });
        }

        let name = input.parse()?;

        let mut attrs = Vec::new();
        while !(input.peek(syn::Token![>])
            || (input.peek(syn::Token![/]) && input.peek2(syn::Token![>])))
        {
            attrs.push(input.parse()?);
        }

        let self_closing = input.parse::<Option<syn::Token![/]>>()?.is_some();
        input.parse::<syn::Token![>]>()?;

        Ok(Self::Opening {
            name,
            attrs,
            self_closing,
        })
    }
}

impl Parse for ast::Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![<])
            && input.peek2(syn::Token![!])
            && input.peek3(kw::DOCTYPE)
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

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let buffer = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let mut nodes = Vec::new();

        while !input.is_empty() {
            let node = input.parse::<ast::Node>()?;
            nodes.push(node);
        }

        Ok(Self { buffer, nodes })
    }
}

impl Parse for FormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            let node = input.parse::<ast::Node>()?;
            nodes.push(node);
        }

        Ok(Self { nodes })
    }
}
impl Parse for ConcatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            let node = input.parse::<ast::Node>()?;
            nodes.push(node);
        }

        Ok(Self { nodes })
    }
}
