use indoc::formatdoc;
use syn::Result;

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_summary_mock(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::SummaryMock { summary },
            ..
        }) = &node.kind
        else {
            index += 1;
            continue;
        };

        let span = node.span();
        let style = node.style;

        nodes.insert(
            0,
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: formatdoc! {r#"
                            <div id="doctored-summary-mock">
                                {summary}
                            </div>

                            <style>
                                #doctored-summary-mock {{
                                    display: none;
                                }}
                            </style>
                        "#},
                    span,
                }),
                style,
            },
        );

        // Resolve the node, which is now offset by 3.
        nodes[index + 1].resolve();

        break;
    }

    Ok(())
}
