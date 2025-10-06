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

    let mut name = None;
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

            if name.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "name cannot be specified more than once",
                ));
            }

            head = meta.path().is_ident("head");
        } else if meta.path().is_ident("strip") {
            match meta {
                Meta::Path(_) => modifiers.push(ClipboardModifier::Strip),
                Meta::List(list) => {
                    let metas =
                        list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                    for meta in metas {
                        if meta.path().is_ident("left") {
                            let mut prefix = None;

                            match meta {
                                Meta::Path(_) => {}
                                Meta::NameValue(MetaNameValue { value, .. }) => {
                                    let Expr::Lit(ExprLit {
                                        lit: Lit::Str(s), ..
                                    }) = value
                                    else {
                                        return Err(Error::new(
                                            value.span(),
                                            "expected a string literal",
                                        ));
                                    };

                                    prefix.replace(s.value());
                                }
                                Meta::List(_) => {
                                    return Err(Error::new(
                                        meta.span(),
                                        "malformed attribute argument",
                                    ));
                                }
                            }

                            modifiers.push(ClipboardModifier::StripLeft(prefix));
                        } else if meta.path().is_ident("right") {
                            let mut suffix = None;

                            match meta {
                                Meta::Path(_) => {}
                                Meta::NameValue(MetaNameValue { value, .. }) => {
                                    let Expr::Lit(ExprLit {
                                        lit: Lit::Str(s), ..
                                    }) = value
                                    else {
                                        return Err(Error::new(
                                            value.span(),
                                            "expected a string literal",
                                        ));
                                    };

                                    suffix.replace(s.value());
                                }
                                Meta::List(_) => {
                                    return Err(Error::new(
                                        meta.span(),
                                        "malformed attribute argument",
                                    ));
                                }
                            }

                            modifiers.push(ClipboardModifier::StripRight(suffix));
                        } else {
                            return Err(Error::new(meta.span(), "invalid attribute argument"));
                        }
                    }
                }
                _ => return Err(Error::new(meta.span(), "malformed attribute argument")),
            }
        } else if meta.path().is_ident("push") {
            let metas = meta
                .require_list()?
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in metas {
                if meta.path().is_ident("left") {
                    let value = &meta.require_name_value()?.value;

                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = value
                    else {
                        return Err(Error::new(value.span(), "expected a string literal"));
                    };

                    modifiers.push(ClipboardModifier::PushLeft(s.value()));
                } else if meta.path().is_ident("right") {
                    let value = &meta.require_name_value()?.value;

                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = value
                    else {
                        return Err(Error::new(value.span(), "expected a string literal"));
                    };

                    modifiers.push(ClipboardModifier::PushRight(s.value()));
                } else {
                    return Err(Error::new(meta.span(), "invalid attribute argument"));
                }
            }
        } else {
            return Err(Error::new(meta.span(), "invalid attribute argument"));
        }
    }

    let Some(name) = name else {
        return Err(Error::new(meta.span(), "expected a name"));
    };

    let kind = if head {
        ArgumentKind::ClipboardCopyHead { name, modifiers }
    } else {
        if !modifiers.is_empty() {
            return Err(Error::new(meta.span(), "tail cannot have modifiers"));
        }

        ArgumentKind::ClipboardCopyTail { name }
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
