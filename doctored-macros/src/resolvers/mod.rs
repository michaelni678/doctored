use std::collections::HashMap;

use syn::Result;

use crate::{
    resolvers::{
        clipboard::{copy::resolve_clipboard_copy, paste::resolve_clipboard_paste},
        highlight::resolve_highlight,
        summary::resolve_summary,
    },
    utilities::nodes::Node,
};

pub mod clipboard;
pub mod highlight;
pub mod summary;

pub fn resolve(nodes: &mut Vec<Node>) -> Result<()> {
    let mut clipboard = HashMap::new();

    resolve_clipboard_copy(nodes, &mut clipboard);
    resolve_clipboard_paste(nodes, clipboard)?;

    resolve_highlight(nodes)?;

    resolve_summary(nodes)?;

    Ok(())
}
