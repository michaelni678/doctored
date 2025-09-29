use syn::Result;

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_hide(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    loop {
        let Some(node) = nodes.get(index) else {
            return Ok(());
        };

        let span = node.span();
        let style = node.style;

        if let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::SummaryHide,
            ..
        }) = node.kind
        {
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

            return Ok(());
        }

        index += 1;
    }
}
