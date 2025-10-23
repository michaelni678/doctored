use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

use crate::{
    doctored::{
        nodes::convert::{convert_attributes_into_nodes, convert_nodes_into_attributes},
        resolvers::{
            clipboard::{copy::resolve_clipboard_copy, paste::resolve_clipboard_paste},
            disregard::resolve_disregard,
            summary::resolve_summary,
            tag::resolve_tag,
        },
    },
    utilities::attributes::visit::visit_attributes,
};

pub mod nodes;
pub mod parsers;
pub mod resolvers;

pub fn doctored(input: TokenStream2) -> Result<TokenStream2> {
    visit_attributes(input, &mut |attrs| {
        let mut clipboard = HashMap::new();

        let mut nodes = convert_attributes_into_nodes(attrs)?;

        #[cfg(feature = "extras")]
        {
            use crate::doctored::resolvers::extras::include::resolve_extras_include;

            resolve_extras_include(&mut nodes)?;
        }

        resolve_clipboard_copy(&mut nodes, &mut clipboard)?;
        resolve_clipboard_paste(&mut nodes, clipboard)?;

        resolve_disregard(&mut nodes)?;

        resolve_summary(&mut nodes)?;

        resolve_tag(&mut nodes)?;

        convert_nodes_into_attributes(nodes)
    })
}
