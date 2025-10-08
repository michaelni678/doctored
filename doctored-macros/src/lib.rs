use proc_macro::TokenStream;
use syn::{Error, parse::Nothing, parse_macro_input};

use crate::{
    resolvers::{
        clipboard::{copy::resolve_clipboard_copy, paste::resolve_clipboard_paste},
        highlight::resolve_highlight,
        summary::resolve_summary,
        tag::resolve_tag,
    },
    utilities::{
        attributes::visit::visit_attributes,
        context::Context,
        nodes::convert::{convert_attributes_into_nodes, convert_nodes_into_attributes},
    },
};

mod parsers;
mod resolvers;
mod utilities;

#[proc_macro_attribute]
pub fn doctored(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);

    visit_attributes(input.into(), &mut |attrs| {
        let mut nodes = convert_attributes_into_nodes(attrs)?;

        let mut context = Context::default();

        resolve_clipboard_copy(&mut nodes, &mut context)?;
        resolve_clipboard_paste(&mut nodes, &context)?;

        resolve_highlight(&mut nodes)?;

        resolve_summary(&mut nodes)?;

        resolve_tag(&mut nodes)?;

        convert_nodes_into_attributes(nodes)
    })
    .unwrap_or_else(Error::into_compile_error)
    .into()
}
