use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use tiny_rsx as rsx;

pub struct BracedValueWithSpecs {
    pub value: syn::Expr,
    pub specs: Option<TokenStream>,
    pub escape_flag: bool,
}

impl ToTokens for BracedValueWithSpecs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let value = &self.value;

        if self.escape_flag {
            quote! {
                ::fhtml::escape(#value)
            }
            .to_tokens(tokens)
        } else {
            value.to_tokens(tokens)
        }
    }
}

pub enum Value {
    Text(syn::LitStr),
    Braced(BracedValueWithSpecs),
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Text(text) => text.to_tokens(tokens),
            Value::Braced(braced) => braced.to_tokens(tokens),
        }
    }
}

impl TryFrom<rsx::Value> for Value {
    type Error = syn::Error;

    fn try_from(value: rsx::Value) -> Result<Self, Self::Error> {
        match value {
            rsx::Value::Text(text) => Ok(Self::Text(text)),
            rsx::Value::Braced(braced) => {
                Ok(Self::Braced(syn::parse2(braced)?))
            }
        }
    }
}

pub struct Attr {
    pub name: rsx::DashIdent,
    pub value: Value,
}

impl TryFrom<rsx::Attr> for Attr {
    type Error = syn::Error;

    fn try_from(attr: rsx::Attr) -> Result<Self, Self::Error> {
        Ok(Self {
            name: attr.name,
            value: attr.value.try_into()?,
        })
    }
}

pub enum Tag {
    Start {
        name: rsx::DashIdent,
        attributes: Vec<Attr>,
        self_closing: bool,
    },
    End {
        name: rsx::DashIdent,
    },
}

impl TryFrom<rsx::Tag> for Tag {
    type Error = syn::Error;

    fn try_from(tag: rsx::Tag) -> Result<Self, Self::Error> {
        match tag {
            rsx::Tag::Start {
                name,
                attributes,
                self_closing,
            } => Ok(Self::Start {
                name,
                attributes: attributes
                    .into_iter()
                    .map(|attr| attr.try_into())
                    .collect::<Result<Vec<_>, _>>()?,
                self_closing,
            }),
            rsx::Tag::End { name } => Ok(Self::End { name }),
        }
    }
}

pub enum Segment {
    Doctype(rsx::Doctype),
    Tag(Tag),
    Value(Value),
}

impl TryFrom<rsx::Segment> for Segment {
    type Error = syn::Error;

    fn try_from(segment: rsx::Segment) -> Result<Self, Self::Error> {
        match segment {
            rsx::Segment::Doctype(doctype) => Ok(Self::Doctype(doctype)),
            rsx::Segment::Tag(tag) => Ok(Self::Tag(tag.try_into()?)),
            rsx::Segment::Value(value) => Ok(Self::Value(value.try_into()?)),
        }
    }
}

pub struct Template {
    pub segments: Vec<Segment>,
    pub values: Vec<Value>,
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_string().to_tokens(tokens);
    }
}
