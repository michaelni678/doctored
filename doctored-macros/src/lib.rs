use proc_macro::TokenStream;
use syn::{Error, parse::Nothing, parse_macro_input};

use crate::utilities::{
    attributes::visit::visit_attributes,
    nodes::convert::{convert_attributes_into_nodes, convert_nodes_into_attributes},
};

mod utilities;

#[proc_macro_attribute]
pub fn doctored(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);

    visit_attributes(input.into(), &mut |attrs| {
        let mut nodes = convert_attributes_into_nodes(attrs)?;

        convert_nodes_into_attributes(nodes)
    })
    .unwrap_or_else(Error::into_compile_error)
    .into()
}
