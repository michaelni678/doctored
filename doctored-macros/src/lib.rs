use proc_macro::TokenStream;
use syn::{Error, parse::Nothing, parse_macro_input};

mod doctored;
mod utilities;

#[proc_macro_attribute]
pub fn doctored(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);

    doctored::doctored(input.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
