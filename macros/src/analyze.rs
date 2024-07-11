use crate::ast;

pub(crate) fn analyze_nodes<V>(nodes: &[ast::Node<V>]) -> syn::Result<()> {
    analyze_node_tree(nodes)?;
    Ok(())
}

fn analyze_node_tree<V>(nodes: &[ast::Node<V>]) -> syn::Result<()> {
    let mut stack = Vec::new();

    for node in nodes {
        if let ast::Node::Tag(tag) = node {
            match &tag.kind {
                ast::TagKind::Opening { self_closing, .. } => {
                    if !self_closing {
                        stack.push(tag);
                    }
                }
                ast::TagKind::Closing { name } => {
                    if let Some(stack_tag) = stack.pop() {
                        if name != stack_tag.name() {
                            return Err(syn::Error::new(
                                tag.span,
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
                            tag.span,
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
            stack_tag.span,
            format_args!(
                "opening tag has no corresponding closing <{}> tag",
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
    fn matching_opening_and_closing_tag() {
        analyze_node_tree(&[
            ast::Tag::<ast::LitValue> {
                kind: ast::TagKind::Opening {
                    name: dash_ident!(foo),
                    attrs: vec![],
                    self_closing: false,
                },
                span: Span::mixed_site(),
            }
            .into(),
            ast::Tag::<ast::LitValue> {
                kind: ast::TagKind::Closing {
                    name: dash_ident!(foo),
                },
                span: Span::mixed_site(),
            }
            .into(),
        ])
        .expect(
            "an opening and closing tag with the same name should be valid",
        );
    }

    #[test]
    fn single_self_closing_tag() {
        analyze_node_tree(&[ast::Tag::<ast::LitValue> {
            kind: ast::TagKind::Opening {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing: true,
            },
            span: Span::mixed_site(),
        }
        .into()])
        .expect("a single self-closing tag should be valid");
    }

    #[test]
    fn single_opening_tag() {
        analyze_node_tree(&[ast::Tag {
            kind: ast::TagKind::Opening::<ast::LitValue> {
                name: dash_ident!(foo),
                attrs: vec![],
                self_closing: false,
            },
            span: Span::mixed_site(),
        }
        .into()])
        .expect_err("a single opening tag should be invalid");
    }

    #[test]
    fn single_closing_tag() {
        analyze_node_tree(&[ast::Tag {
            kind: ast::TagKind::Closing::<ast::LitValue> {
                name: dash_ident!(foo),
            },
            span: Span::mixed_site(),
        }
        .into()])
        .expect_err("a single closing tag should be invalid");
    }

    #[test]
    fn mismatched_opening_and_closing_tag() {
        analyze_node_tree(&[
            ast::Tag {
                kind: ast::TagKind::Opening::<ast::LitValue> {
                    name: dash_ident!(foo),
                    attrs: vec![],
                    self_closing: false,
                },
                span: Span::mixed_site(),
            }
            .into(),
            ast::Tag {
                kind: ast::TagKind::Closing::<ast::LitValue> {
                    name: dash_ident!(bar),
                },
                span: Span::mixed_site(),
            }
            .into(),
        ])
        .expect_err(
            "a mismatch between opening and closing tag should be invalid",
        );
    }
}
