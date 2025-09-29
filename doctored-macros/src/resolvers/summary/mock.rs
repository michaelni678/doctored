use syn::Result;

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_mock(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    loop {
        let Some(node) = nodes.get(index) else {
            return Ok(());
        };

        let span = node.span();
        let style = node.style;

        if let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::SummaryMock(string),
            ..
        }) = &node.kind
        {
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

            // Resolve the node, which is now offset by 3.
            nodes[index + 3].resolve();

            return Ok(());
        }

        index += 1;
    }
}
