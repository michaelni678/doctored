use syn::{AttrStyle, Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, spanned::Spanned};

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn parse_highlight(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let theme = match &meta {
        Meta::Path(_) => None,
        Meta::NameValue(MetaNameValue { value, .. }) => {
            let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            else {
                return Err(Error::new(value.span(), "expected a string literal"));
            };

            Some(s.value())
        }
        Meta::List(_) => {
            return Err(Error::new(meta.span(), "malformed attribute argument"));
        }
    };

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Highlight { theme },
            resolved: false,
            span: meta.span(),
        }),
        style,
    });

    Ok(())
}
