use syn::Result;

use crate::{
    resolvers::summary::{hide::resolve_summary_hide, mock::resolve_summary_mock},
    utilities::context::Context,
};

pub mod hide;
pub mod mock;

pub fn resolve_summary(context: &mut Context) -> Result<()> {
    resolve_summary_hide(context)?;
    resolve_summary_mock(context)?;

    Ok(())
}
