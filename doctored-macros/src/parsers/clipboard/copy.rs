use syn::{
    AttrStyle, Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, Token,
    punctuated::Punctuated, spanned::Spanned,
};

use crate::{
    resolvers::clipboard::ClipboardModifier,
    utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind},
};

pub fn parse_clipboard_copy(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    let mut tag = None;
    let mut head = false;
    let mut modifiers = Vec::new();

    for meta in metas {
        if meta.path().is_ident("head") || meta.path().is_ident("tail") {
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

            head = meta.path().is_ident("head");
        } else if meta.path().is_ident("strip") {
            meta.require_path_only()?;

            modifiers.push(ClipboardModifier::Strip);
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

    let kind = if head {
        ArgumentKind::ClipboardCopyHead { tag, modifiers }
    } else {
        if !modifiers.is_empty() {
            return Err(Error::new(meta.span(), "tail cannot have modifiers"));
        }

        ArgumentKind::ClipboardCopyTail { tag }
    };

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind,
            span: meta.span(),
            resolved: false,
        }),
        style,
    });

    Ok(())
}
