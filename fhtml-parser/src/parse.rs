use syn::{ext::IdentExt as _, parse::ParseStream, punctuated::Punctuated};

use crate::ast;

mod kw {
    syn::custom_keyword!(DOCTYPE);
    syn::custom_keyword!(html);
}

pub struct ParserOptions {}

pub struct Parser<'a> {
    input: ParseStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: ParseStream<'a>) -> Self {
        Self { input }
    }

    pub fn parse_dash_ident(&self) -> syn::Result<ast::DashIdent> {
        // Parse a non-empty sequence of identifiers separated by dashes.
        let inner = Punctuated::<syn::Ident, syn::Token![-]>::parse_separated_nonempty_with(
            self.input,
            syn::Ident::parse_any,
        )?;

        Ok(ast::DashIdent(inner))
    }

    pub fn parse_doctype(&self) -> syn::Result<ast::Doctype> {
        self.input.parse::<syn::Token![<]>()?;
        self.input.parse::<syn::Token![!]>()?;
        self.input.parse::<kw::DOCTYPE>()?;
        self.input.parse::<kw::html>()?;
        self.input.parse::<syn::Token![>]>()?;

        Ok(ast::Doctype)
    }

    pub fn parse_value(&self) -> syn::Result<ast::Value> {
        let lookahead = self.input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            Ok(ast::Value::LitStr(self.input.parse()?))
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in self.input);
            Ok(ast::Value::Expr(content.parse()?))
        } else {
            Err(lookahead.error())
        }
    }

    pub fn parse_attr(&self) -> syn::Result<ast::Attr> {
        let name = self.parse_dash_ident()?;
        self.input.parse::<syn::Token![=]>()?;
        let value = self.parse_value()?;

        Ok(ast::Attr { name, value })
    }

    pub fn parse_tag(&self) -> syn::Result<ast::Tag> {
        self.input.parse::<syn::Token![<]>()?;

        if self.input.parse::<Option<syn::Token![/]>>()?.is_some() {
            let name = self.parse_dash_ident()?;
            self.input.parse::<syn::Token![>]>()?;

            return Ok(ast::Tag::Closing { name });
        }

        let name = self.parse_dash_ident()?;

        let mut attrs = Vec::new();
        while !(self.input.peek(syn::Token![>])
            || (self.input.peek(syn::Token![/])
                && self.input.peek2(syn::Token![>])))
        {
            attrs.push(self.parse_attr()?);
        }

        let void = self.input.parse::<Option<syn::Token![/]>>()?.is_some();
        self.input.parse::<syn::Token![>]>()?;

        Ok(ast::Tag::Opening { name, attrs, void })
    }

    pub fn parse_node(&self) -> syn::Result<ast::Node> {
        let lookahead = self.input.lookahead1();
        if lookahead.peek(syn::Token![<])
            && self.input.peek2(syn::Token![!])
            && self.input.peek3(kw::DOCTYPE)
        {
            Ok(ast::Node::Doctype(self.parse_doctype()?))
        } else if lookahead.peek(syn::Token![<]) {
            Ok(ast::Node::Tag(self.parse_tag()?))
        } else if lookahead.peek(syn::LitStr)
            || lookahead.peek(syn::token::Brace)
        {
            Ok(ast::Node::Value(self.parse_value()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn parse(input: ParseStream<'_>) -> syn::Result<Vec<ast::Node>> {
    let parser = Parser::new(input);
    let mut nodes = Vec::new();
    while !input.is_empty() {
        nodes.push(parser.parse_node()?)
    }
    Ok(nodes)
}
