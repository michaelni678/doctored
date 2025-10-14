use syn::{AttrStyle, Error, Expr, ExprLit, Lit, Meta, Result, spanned::Spanned};

use crate::doctored::{
    nodes::{ArgumentKind, ArgumentNode, Node, NodeKind},
    resolvers::extras::include::IncludeKind,
};

pub fn parse_extras_include(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    // Validate the meta is a name-value pair.
    let value = &meta.require_name_value()?.value;

    let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = value
    else {
        return Err(Error::new(value.span(), "expected a string literal"));
    };

    let filename = s.value();

    let kind = if meta.path().is_ident("attributes") {
        IncludeKind::Attributes
    } else if meta.path().is_ident("documentation") {
        IncludeKind::Documentation
    } else {
        return Err(Error::new(meta.span(), "invalid attribute argument"));
    };

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Include { kind, filename },
            resolved: false,
            span: meta.span(),
        }),
        style,
    });

    Ok(())
}
