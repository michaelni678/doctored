use syn::{Error, Result};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_hide(nodes: &mut Vec<Node>) -> Result<()> {
    let mut extracted = nodes.extract_if(.., |node| {
        matches!(
            node.kind,
            NodeKind::Argument(ArgumentNode {
                kind: ArgumentKind::SummaryHide,
                ..
            })
        )
    });

    let Some(node) = extracted.next() else {
        return Ok(());
    };

    if let Some(node) = extracted.next() {
        return Err(Error::new(node.span(), "redundant attribute argument"));
    }

    drop(extracted);

    nodes.insert(
        0,
        Node {
            kind: NodeKind::Documentation(DocumentationNode {
                string: String::from("<!-- -->"),
                span: node.span(),
            }),
            style: node.style,
        },
    );

    Ok(())
}
