use syn::Result;

use crate::doctored::{
    nodes::Node,
    resolvers::summary::{hide::resolve_summary_hide, mock::resolve_summary_mock},
};

pub mod hide;
pub mod mock;

pub fn resolve_summary(nodes: &mut Vec<Node>) -> Result<()> {
    resolve_summary_hide(nodes)?;
    resolve_summary_mock(nodes)?;

    Ok(())
}
