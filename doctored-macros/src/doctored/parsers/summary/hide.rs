use syn::{AttrStyle, Meta, Result, spanned::Spanned};

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn parse_summary_hide(nodes: &mut Vec<Node>, attr_style: AttrStyle, meta: Meta) -> Result<()> {
    // Validate the meta is a path.
    meta.require_path_only()?;

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::SummaryHide,
            resolved: false,
            span: meta.span(),
        }),
        attr_style,
    });

    Ok(())
}
