use proc_macro2::{Group, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{
    Attribute, Result, Token,
    parse::{ParseStream, Parser},
};

/// Visits sets of attributes in `input` and calls `f` on them.
pub fn visit_attributes<F>(input: TokenStream2, f: &mut F) -> Result<TokenStream2>
where
    F: FnMut(Vec<Attribute>) -> Result<Vec<Attribute>>,
{
    let mut expansion = TokenStream2::new();

    Parser::parse2(
        |input: ParseStream| {
            while !input.is_empty() {
                if input.peek(Token![#])
                    && let Ok(attrs) = if input.peek2(Token![!]) {
                        Attribute::parse_inner(input)
                    } else {
                        Attribute::parse_outer(input)
                    }
                {
                    let attrs = f(attrs)?;

                    expansion.extend(quote!(#(#attrs)*));
                } else {
                    let mut tt = input.parse()?;

                    // If the token tree is a group, resolve the inner tokens.
                    if let TokenTree::Group(group) = tt {
                        let delimiter = group.delimiter();
                        let tokens = visit_attributes(group.stream(), f)?;

                        tt = TokenTree::Group(Group::new(delimiter, tokens));
                    }

                    expansion.extend(quote!(#tt));
                }
            }

            Ok(())
        },
        input,
    )?;

    Ok(expansion)
}
