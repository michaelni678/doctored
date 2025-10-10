use syn::Result;

use crate::utilities::{
    context::Context,
    nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind},
};

pub fn resolve_summary_hide(context: &mut Context) -> Result<()> {
    let mut index = 0;

    while let Some(node) = context.nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::SummaryHide,
            ..
        }) = node.kind
        else {
            index += 1;
            continue;
        };

        let attr_style = node.attr_style;
        let span = node.span();

        context.nodes.insert(
            0,
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: String::from("<!-- -->"),
                    span,
                }),
                attr_style,
            },
        );

        // Resolve the node, which is now offset by 1.
        context.nodes[index + 1].resolve();

        break;
    }

    Ok(())
}
