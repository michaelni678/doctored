use syn::{Error, Result};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_mock(nodes: &mut Vec<Node>) -> Result<()> {
    // Extract nodes to resolve.
    let mut unresolved = nodes.extract_if(.., |node| {
        matches!(
            node.kind,
            NodeKind::Argument(ArgumentNode {
                kind: ArgumentKind::SummaryMock(_),
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

    let NodeKind::Argument(ArgumentNode {
        kind: ArgumentKind::SummaryMock(string),
        ..
    }) = node.kind
    else {
        unreachable!();
    };

    nodes.splice(
        0..0,
        [
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: format!("<div id=\"summary_mock\">{string}</div>"),
                    span,
                }),
                style,
            },
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: String::new(),
                    span,
                }),
                style,
            },
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: String::from("<style>#summary_mock { display: none; }</style>"),
                    span,
                }),
                style,
            },
        ],
    );

    Ok(())
}
