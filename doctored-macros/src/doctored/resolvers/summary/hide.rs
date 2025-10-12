use syn::Result;

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_hide(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::SummaryHide,
            ..
        }) = node.kind
        else {
            index += 1;
            continue;
        };

        let style = node.style;
        let span = node.span();

        nodes.insert(
            0,
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: String::from("<!-- -->"),
                    span,
                }),
                style,
            },
        );

        // Resolve the node, which is now offset by 1.
        nodes[index + 1].resolve();

        break;
    }

    Ok(())
}
