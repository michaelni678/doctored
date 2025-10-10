use syn::{AttrStyle, Error, Meta, Result, Token, punctuated::Punctuated, spanned::Spanned};

use crate::{
    parsers::summary::{hide::parse_summary_hide, mock::parse_summary_mock},
    utilities::nodes::Node,
};

pub mod hide;
pub mod mock;

pub fn parse_summary(nodes: &mut Vec<Node>, attr_style: AttrStyle, meta: Meta) -> Result<()> {
    let metas = meta
        .require_list()?
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

    for meta in metas {
        if meta.path().is_ident("hide") {
            parse_summary_hide(nodes, attr_style, meta)?;
        } else if meta.path().is_ident("mock") {
            parse_summary_mock(nodes, attr_style, meta)?;
        } else {
            return Err(Error::new(meta.span(), "invalid attribute argument"));
        }
    }

    Ok(())
}
