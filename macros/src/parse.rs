use std::fmt::Write;

use proc_macro2::Span;
use quote::ToTokens;
use syn::ext::IdentExt as _;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::analyze::analyze_nodes;
use crate::{ast, lower_ast, ConcatInput, FormatArgsInput};

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(doctype);
    syn::custom_keyword!(html);
}

impl Parse for ast::DashIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse a non-empty sequence of identifiers separated by dashes.
        let inner = Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(
            input,
            syn::Ident::parse_any,
        )?;

        Ok(Self {
            span: inner.span(),
            inner,
        })
    }
}

impl Parse for ast::Doctype {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<syn::Token![<]>()?.span;
        input.parse::<syn::Token![!]>()?;
        if input.peek(kw::doctype) {
            input.parse::<kw::doctype>()?;
        } else {
            input.parse::<kw::DOCTYPE>()?;
        }
        input.parse::<kw::html>()?;
        let end_span = input.parse::<syn::Token![>]>()?.span;

        Ok(Self {
            span: start_span.join(end_span).unwrap_or(Span::call_site()),
        })
    }
}

impl<V: Parse> Parse for ast::Value<V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let inner = input.parse()?;

        Ok(Self { span, inner })
    }
}

impl Parse for ast::LitValue {
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

impl Parse for ast::PlaceholderValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            Ok(Self::LitStr(input.parse()?))
        } else if lookahead.peek(syn::token::Brace) {
            let content;

            syn::braced!(content in input);

            let value = content.parse()?;
            let mut specs = None;

            if content.parse::<Option<syn::Token![:]>>()?.is_some() {
                specs = Some(content.parse()?);
            }

            Ok(Self::Expr { value, specs })
        } else {
            Err(lookahead.error())
        }
    }
}

impl<V: Parse> Parse for ast::Attr<V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<ast::DashIdent>()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse::<ast::Value<V>>()?;

        Ok(Self {
            span: name.span.join(value.span).unwrap_or(Span::call_site()),
            name,
            value,
        })
    }
}

impl<V: Parse> Parse for ast::Tag<V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<syn::Token![<]>()?.span;

        if input.parse::<Option<syn::Token![/]>>()?.is_some() {
            let name = input.parse()?;
            let end_span = input.parse::<syn::Token![>]>()?.span;

            return Ok(Self {
                kind: ast::TagKind::Closing { name },
                span: start_span.join(end_span).unwrap_or(Span::call_site()),
            });
        }

        let name = input.parse()?;

        let mut attrs = Vec::new();
        while !(input.peek(syn::Token![>])
            || (input.peek(syn::Token![/]) && input.peek2(syn::Token![>])))
        {
            attrs.push(input.parse()?);
        }

        let self_closing = input.parse::<Option<syn::Token![/]>>()?.is_some();
        let end_span = input.parse::<syn::Token![>]>()?.span;

        Ok(Self {
            kind: ast::TagKind::Opening {
                name,
                attrs,
                self_closing,
            },
            span: start_span.join(end_span).unwrap_or(Span::call_site()),
        })
    }
}

impl<V: Parse> Parse for ast::Node<V> {
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

impl Parse for FormatArgsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut template = String::new();
        let mut values = Vec::new();
        let mut nodes = Vec::new();

        while !input.is_empty() {
            let node = input.parse::<ast::Node<ast::PlaceholderValue>>()?;
            values.extend(node.get_all_values());
            nodes.push(node);
        }

        analyze_nodes(&nodes)?;

        for node in nodes {
            for token in node.into_node_tokens() {
                let _ = write!(template, "{}", token);
            }
        }

        Ok(Self { template, values })
    }
}

impl Parse for ConcatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = Vec::new();
        let mut acc = String::new();
        let mut nodes = Vec::new();

        while !input.is_empty() {
            let node = input.parse::<ast::Node<ast::LitValue>>()?;
            nodes.push(node);
        }

        analyze_nodes(&nodes)?;

        for node in nodes {
            for token in node.into_node_tokens() {
                match token {
                    lower_ast::NodeToken::AttrValue(v)
                    | lower_ast::NodeToken::Value(v) => {
                        if let ast::LitValue::LitStr(lit) = v {
                            acc.push_str(&lit.value());
                        } else {
                            segments.push(acc.to_token_stream());
                            segments.push(v.to_token_stream());
                            acc.clear();
                        }
                    }
                    _ => {
                        let _ = write!(acc, "{}", token);
                    }
                }
            }
        }

        if !acc.is_empty() {
            segments.push(acc.to_token_stream());
        }

        Ok(Self { segments })
    }
}
