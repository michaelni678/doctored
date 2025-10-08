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
        let nodes = convert_attributes_into_nodes(attrs)?;

        let mut context = Context::default();
        context.nodes.extend(nodes);

        resolve_clipboard_copy(&mut context)?;
        resolve_clipboard_paste(&mut context)?;

        resolve_highlight(&mut context)?;

        resolve_summary(&mut context)?;

        resolve_tag(&mut context)?;

        convert_nodes_into_attributes(context.nodes)
    })
    .unwrap_or_else(Error::into_compile_error)
    .into()
}
