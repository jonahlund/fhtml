use syn::spanned::Spanned;

use crate::ast;

pub(crate) fn analyze_nodes<V: Spanned>(
    nodes: &[ast::Node<V>],
) -> syn::Result<()> {
    check_node_tree(nodes)?;
    check_duplicate_attrs(nodes)?;
    Ok(())
}

fn check_duplicate_attrs<V: Spanned>(
    nodes: &[ast::Node<V>],
) -> syn::Result<()> {
    for node in nodes {
        if let ast::Node::Tag(ast::Tag::Opening { attrs, .. }) = node {
            for attr in attrs.iter().rev() {
                if attrs.iter().filter(|a| attr.name == a.name).count() > 1 {
                    return Err(syn::Error::new(
                        attr.span(),
                        format_args!("duplicate attribute `{}`", attr.name),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn check_node_tree<V: Spanned>(nodes: &[ast::Node<V>]) -> syn::Result<()> {
    let mut stack = Vec::new();

    for node in nodes {
        if let ast::Node::Tag(tag) = node {
            match &tag {
                ast::Tag::Opening { .. } => {
                    if !tag.is_self_closing() {
                        stack.push(tag);
                    }
                }
                ast::Tag::Closing { name } => {
                    if let Some(stack_tag) = stack.pop() {
                        if name != stack_tag.name() {
                            return Err(syn::Error::new(
                                tag.span(),
                                format_args!(
                                    "closing tag mismatch, expected </{}>, \
                                     found </{}>",
                                    stack_tag.name(),
                                    name,
                                ),
                            ));
                        }
                    } else {
                        return Err(syn::Error::new(
                            tag.span(),
                            format_args!(
                                "closing tag has no corresponding opening \
                                 <{}> tag",
                                name
                            ),
                        ));
                    }
                }
            }
        }
    }

    if let Some(stack_tag) = stack.pop() {
        return Err(syn::Error::new(
            stack_tag.span(),
            format_args!(
                "opening tag has no corresponding closing </{}> tag",
                stack_tag.name()
            ),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    use super::*;

    macro_rules! dash_ident {
        ($($arg:tt)*) => {
            syn::parse2::<crate::ast::DashIdent>(
                quote::quote! { $($arg)* }
            ).unwrap()
        }
    }

    #[test]
    fn opening_and_closing_tags() {
        check_node_tree(&[
            ast::Node::Tag(ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing_slash: None,
            }),
            ast::Node::Tag(ast::Tag::<ast::LitValue>::Closing {
                name: dash_ident!(foo),
            }),
        ])
        .expect(
            "an opening and closing tag with the same name should be allowed",
        );
    }

    #[test]
    fn self_closing_tag() {
        check_node_tree(&[ast::Node::Tag(
            ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing_slash: Some(syn::Token![/]([Span::call_site()])),
            },
        )])
        .expect("a self-closing tag should be allowed");
    }

    #[test]
    fn single_opening_tag() {
        check_node_tree(&[ast::Node::Tag(
            ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing_slash: None,
            },
        )])
        .expect_err("a single opening tag should be disallowed");
    }

    #[test]
    fn single_closing_tag() {
        check_node_tree(&[ast::Node::Tag(
            ast::Tag::<ast::LitValue>::Closing {
                name: dash_ident!(foo),
            },
        )])
        .expect_err("a single closing tag should be disallowed");
    }

    #[test]
    fn mismatched_opening_and_closing_tag() {
        check_node_tree(&[
            ast::Node::Tag(ast::Tag::<ast::LitValue>::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing_slash: None,
            }),
            ast::Node::Tag(ast::Tag::<ast::LitValue>::Closing {
                name: dash_ident!(bar),
            }),
        ])
        .expect_err(
            "a mismatch between opening and closing tag should be disallowed",
        );
    }

    #[test]
    fn duplicate_attrs() {
        check_duplicate_attrs(&[ast::Node::Tag(ast::Tag::Opening {
            name: dash_ident!(foo),
            attrs: vec![
                ast::Attr {
                    name: dash_ident!(bar),
                    eq_sep: syn::Token![=]([Span::call_site()]),
                    value: ast::LitValue::LitStr(syn::LitStr::new(
                        "",
                        Span::call_site(),
                    )),
                },
                ast::Attr {
                    name: dash_ident!(bar),
                    eq_sep: syn::Token![=]([Span::call_site()]),
                    value: ast::LitValue::LitStr(syn::LitStr::new(
                        "",
                        Span::call_site(),
                    )),
                },
            ],
            self_closing_slash: Some(syn::Token![/]([Span::call_site()])),
        })])
        .expect_err("duplicate attribute should be disallowed");
    }
}
