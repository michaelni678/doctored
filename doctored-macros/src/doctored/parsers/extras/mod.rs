use syn::{AttrStyle, Error, Meta, Result, Token, punctuated::Punctuated, spanned::Spanned};

use crate::doctored::{nodes::Node, parsers::extras::include::parse_extras_include};

pub mod include;

pub fn parse_extras(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    for meta in metas {
        if meta.path().is_ident("include") {
            parse_extras_include(nodes, style, meta)?;
        } else {
            return Err(Error::new(meta.span(), "invalid attribute argument"));
        }
    }

    Ok(())
}
