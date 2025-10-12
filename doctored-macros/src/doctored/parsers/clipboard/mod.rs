use syn::{AttrStyle, Error, Meta, Result, Token, punctuated::Punctuated, spanned::Spanned};

use crate::doctored::{
    nodes::Node,
    parsers::clipboard::{copy::parse_clipboard_copy, paste::parse_clipboard_paste},
};

pub mod copy;
pub mod paste;

pub fn parse_clipboard(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    for meta in metas {
        if meta.path().is_ident("copy") {
            parse_clipboard_copy(nodes, style, meta)?;
        } else if meta.path().is_ident("paste") {
            parse_clipboard_paste(nodes, style, meta)?;
        } else {
            return Err(Error::new(meta.span(), "invalid attribute argument"));
        }
    }

    Ok(())
}
