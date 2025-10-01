use syn::{
    AttrStyle, Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, Token,
    punctuated::Punctuated, spanned::Spanned,
};

use crate::{
    resolvers::clipboard::ClipboardModifier,
    utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind},
};

pub fn parse_clipboard_paste(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    let mut tag = None;
    let mut modifiers = Vec::new();

    for meta in metas {
        if meta.path().is_ident("tag") {
            let value = &meta.require_name_value()?.value;

            let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            else {
                return Err(Error::new(value.span(), "expected a string literal"));
            };

            if tag.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "tag cannot be specified more than once",
                ));
            }
        } else if meta.path().is_ident("lstrip") {
            let mut prefix = None;
            match meta {
                Meta::Path(_) => {}
                Meta::NameValue(MetaNameValue { value, .. }) => {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = value
                    else {
                        return Err(Error::new(value.span(), "expected a string literal"));
                    };

                    prefix.replace(s.value());
                }
                Meta::List(_) => {
                    return Err(Error::new(meta.span(), "malformed attribute argument"));
                }
            }

            modifiers.push(ClipboardModifier::StripStart(prefix));
        } else if meta.path().is_ident("lpush") {
            let value = &meta.require_name_value()?.value;

            let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            else {
                return Err(Error::new(value.span(), "expected a string literal"));
            };

            modifiers.push(ClipboardModifier::PushStart(s.value()));
        } else {
            return Err(Error::new(meta.span(), "invalid attribute argument"));
        }
    }

    let Some(tag) = tag else {
        return Err(Error::new(meta.span(), "expected a tag"));
    };

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::ClipboardPaste { tag, modifiers },
            span: meta.span(),
            resolved: false,
        }),
        style,
    });

    Ok(())
}
