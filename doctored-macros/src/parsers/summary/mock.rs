use syn::{AttrStyle, Error, Expr, ExprLit, Lit, Meta, Result, spanned::Spanned};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn parse_summary_mock(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    let value = &meta.require_name_value()?.value;

    if let Expr::Lit(ExprLit { lit, .. }) = value
        && let Lit::Str(s) = lit
    {
        let string = s.value();

        nodes.push(Node {
            kind: NodeKind::Argument(ArgumentNode {
                kind: ArgumentKind::SummaryMock(string),
                span: meta.span(),
                resolved: false,
            }),
            style,
        })
    } else {
        return Err(Error::new(value.span(), "expected a string literal"));
    }

    Ok(())
}
