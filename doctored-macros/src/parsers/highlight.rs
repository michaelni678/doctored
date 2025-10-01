use syn::{AttrStyle, Meta, Result, spanned::Spanned};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn parse_highlight(nodes: &mut Vec<Node>, style: AttrStyle, meta: Meta) -> Result<()> {
    // Validate the meta is a path.
    meta.require_path_only()?;

    nodes.push(Node {
        kind: NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Highlight,
            span: meta.span(),
            resolved: false,
        }),
        style,
    });

    Ok(())
}
