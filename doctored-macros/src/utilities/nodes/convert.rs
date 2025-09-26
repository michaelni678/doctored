use quote::quote;
use syn::{
    AttrStyle, Attribute, Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, Token,
    parse_quote, punctuated::Punctuated, spanned::Spanned,
};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

/// Converts attributes into nodes.
pub fn convert_attributes_into_nodes(attrs: Vec<Attribute>) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();

    for mut attr in attrs {
        let style = attr.style;

        if attr.path().is_ident("doc")
            && let Meta::NameValue(MetaNameValue { value, .. }) = &attr.meta
            && let Expr::Lit(ExprLit { lit, .. }) = value
            && let Lit::Str(s) = lit
        {
            let string = s.value();
            let span = s.span();

            nodes.push(Node {
                kind: NodeKind::Documentation(DocumentationNode { string, span }),
                style,
            });
        } else if attr.path().is_ident("doc")
            && let Ok(metas) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        {
            let mut unrelated = Vec::new();

            for meta in metas {
                if meta.path().is_ident("summary") {
                    let metas = meta
                        .require_list()?
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                    for meta in metas {
                        if meta.path().is_ident("hide") {
                            // Validate the meta is a path.
                            meta.require_path_only()?;

                            nodes.push(Node {
                                kind: NodeKind::Argument(ArgumentNode {
                                    kind: ArgumentKind::SummaryHide,
                                    span: meta.span(),
                                }),
                                style,
                            });
                        } else {
                            return Err(Error::new(meta.span(), "invalid attribute argument"));
                        }
                    }
                } else {
                    unrelated.push(meta);
                }
            }

            // Check if some built-in documentation arguments were found.
            if !unrelated.is_empty() {
                let Meta::List(list) = &mut attr.meta else {
                    unreachable!();
                };

                // Add the built-in documentation arguments back to the original attribute and
                // push it as an unrelated node.
                list.tokens = quote!(#(#unrelated),*);

                nodes.push(Node {
                    kind: NodeKind::Unrelated(attr),
                    style,
                });
            }
        } else {
            nodes.push(Node {
                kind: NodeKind::Unrelated(attr),
                style,
            });
        }
    }

    Ok(nodes)
}

/// Converts nodes into attributes.
pub fn convert_nodes_into_attributes(nodes: Vec<Node>) -> Result<Vec<Attribute>> {
    let mut attrs = Vec::new();

    for node in nodes {
        match node.kind {
            NodeKind::Argument(ArgumentNode { span, .. }) => {
                return Err(Error::new(span, "couldn't resolve attribute argument"));
            }
            NodeKind::Documentation(DocumentationNode { string, .. }) => {
                attrs.push(match node.style {
                    AttrStyle::Outer => parse_quote!(#[doc = #string]),
                    AttrStyle::Inner(_) => parse_quote!(#![doc = #string]),
                });
            }
            NodeKind::Unrelated(attr) => attrs.push(attr),
        }
    }

    Ok(attrs)
}
