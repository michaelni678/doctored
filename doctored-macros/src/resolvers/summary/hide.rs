use syn::{Error, Result};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_hide(nodes: &mut Vec<Node>) -> Result<()> {
    // Extract nodes to resolve.
    let mut unresolved = nodes.iter_mut().filter(|node| {
        matches!(
            node.kind,
            NodeKind::Argument(ArgumentNode {
                kind: ArgumentKind::SummaryHide,
                ..
            })
        )
    });

    // Check if there are any nodes to resolve.
    let Some(node) = unresolved.next() else {
        return Ok(());
    };

    // Validate there isn't a redundant node.
    if let Some(node) = unresolved.next() {
        return Err(Error::new(node.span(), "redundant attribute argument"));
    }

    drop(unresolved);

    let span = node.span();
    let style = node.style;

    node.resolve();

    let node = Node {
        kind: NodeKind::Documentation(DocumentationNode {
            string: String::from("<!-- -->"),
            span,
        }),
        style,
    };

    nodes.insert(0, node);

    Ok(())
}
