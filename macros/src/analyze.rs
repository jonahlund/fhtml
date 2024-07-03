use crate::ast;

impl ast::Template {
    /// Analyzes the template and returns possible errors/lints
    pub fn analyze(&self) -> syn::Result<()> {
        let mut stack = Vec::new();

        for segment in &self.segments {
            match segment {
                ast::Segment::Doctype(_) => {}
                ast::Segment::Tag(tag) => match &tag.kind {
                    ast::TagKind::Start { name, .. } => {
                        stack.push((name.to_string(), tag.span))
                    }
                    ast::TagKind::End {
                        name: current_tag_name,
                    } => match stack.pop() {
                        Some((expected_tag_name, _)) => {
                            if expected_tag_name != current_tag_name.to_string()
                            {
                                return Err(syn::Error::new(
                                    tag.span,
                                    format!(
                                        "expected `{expected_tag_name}` found \
                                         `{current_tag_name}`"
                                    ),
                                ));
                            }
                        }
                        None => {
                            return Err(syn::Error::new(
                                tag.span,
                                "no matching starting tag found",
                            ));
                        }
                    },
                },
                ast::Segment::Value(_) => {}
            }
        }

        Ok(())
    }
}
