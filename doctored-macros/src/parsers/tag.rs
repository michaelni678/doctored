use syn::{
    AttrStyle, Error, Expr, ExprLit, Lit, Meta, Result, Token, punctuated::Punctuated,
    spanned::Spanned,
};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn parse_tag(
    nodes: &mut Vec<Node>,
    attr_index: usize,
    attr_style: AttrStyle,
    meta: Meta,
) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    let mut text = None;
    let mut href = None;
    let mut color = None;

    for meta in metas {
        let value = &meta.require_name_value()?.value;

        let Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) = value
        else {
            return Err(Error::new(value.span(), "expected a string literal"));
        };

        if meta.path().is_ident("text") {
            if text.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "text cannot be specified more than once",
                ));
            }
        } else if meta.path().is_ident("href") {
            if href.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "href cannot be specified more than once",
                ));
            }
        } else if meta.path().is_ident("color") {
            if color.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "color cannot be specified more than once",
                ));
            }
        } else {
            return Err(Error::new(meta.span(), "invalid attribute argument"));
        }
    }

    let Some(text) = text else {
        return Err(Error::new(meta.span(), "expected text"));
    };

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Tag { text, href, color },
            resolved: false,
            span: meta.span(),
        }),
        attr_index,
        attr_style,
    });

    Ok(())
}
