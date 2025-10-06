use quote::quote;
use syn::{
    AttrStyle, Attribute, Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, Token,
    parse_quote, punctuated::Punctuated,
};

use crate::{
    parsers::{
        clipboard::parse_clipboard, highlight::parse_highlight, summary::parse_summary,
        tag::parse_tag,
    },
    utilities::nodes::{ArgumentNode, DocumentationNode, Node, NodeKind},
};

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
                    parse_summary(&mut nodes, style, meta)?;
                } else if meta.path().is_ident("highlight") {
                    parse_highlight(&mut nodes, style, meta)?;
                } else if meta.path().is_ident("clipboard") {
                    parse_clipboard(&mut nodes, style, meta)?;
                } else if meta.path().is_ident("tag") {
                    parse_tag(&mut nodes, style, meta)?;
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
            NodeKind::Argument(ArgumentNode { resolved, span, .. }) => {
                if !resolved {
                    return Err(Error::new(span, "couldn't resolve attribute argument"));
                }
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
