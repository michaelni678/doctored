use syn::Result;

use crate::{
    resolvers::{highlight::resolve_highlight, summary::resolve_summary},
    utilities::nodes::Node,
};

pub mod highlight;
pub mod summary;

pub fn resolve(nodes: &mut Vec<Node>) -> Result<()> {
    resolve_highlight(nodes)?;
    resolve_summary(nodes)?;

    Ok(())
}
