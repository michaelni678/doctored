use syn::{
    AttrStyle, Error, Expr, ExprLit, Lit, Meta, Result, Token, punctuated::Punctuated,
    spanned::Spanned,
};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn parse_tag(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    let mut text = None;
    let mut href = None;
    let mut text_color = None;
    let mut background_color = None;

    for meta in metas {
        if meta.path().is_ident("text") {
            let value = &meta.require_name_value()?.value;

            let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            else {
                return Err(Error::new(value.span(), "expected a string literal"));
            };

            if text.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "text cannot be specified more than once",
                ));
            }
        } else if meta.path().is_ident("href") {
            let value = &meta.require_name_value()?.value;

            let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            else {
                return Err(Error::new(value.span(), "expected a string literal"));
            };

            if href.replace(s.value()).is_some() {
                return Err(Error::new(
                    meta.span(),
                    "href cannot be specified more than once",
                ));
            }
        } else if meta.path().is_ident("text") {
            let metas = meta
                .require_list()?
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in metas {
                if meta.path().is_ident("color") {
                    let value = &meta.require_name_value()?.value;

                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = value
                    else {
                        return Err(Error::new(value.span(), "expected a string literal"));
                    };

                    if text_color.replace(s.value()).is_some() {
                        return Err(Error::new(
                            meta.span(),
                            "text color cannot be specified more than once",
                        ));
                    }
                } else {
                    return Err(Error::new(meta.span(), "invalid attribute argument"));
                }
            }
        } else if meta.path().is_ident("background") {
            let metas = meta
                .require_list()?
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in metas {
                if meta.path().is_ident("color") {
                    let value = &meta.require_name_value()?.value;

                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = value
                    else {
                        return Err(Error::new(value.span(), "expected a string literal"));
                    };

                    if background_color.replace(s.value()).is_some() {
                        return Err(Error::new(
                            meta.span(),
                            "background color cannot be specified more than once",
                        ));
                    }
                } else {
                    return Err(Error::new(meta.span(), "invalid attribute argument"));
                }
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
            kind: ArgumentKind::Tag {
                text,
                href,
                text_color,
                background_color,
            },
            span: meta.span(),
            resolved: false,
        }),
        style,
    });

    Ok(())
}
