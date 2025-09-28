use syn::{Error, Result};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_highlight(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    // Loop through the nodes and find in-text highlights and turn them into nodes.
    while index < nodes.len() {
        let node = &mut nodes[index];

        let style = node.style;
        let span = node.span();

        if let NodeKind::Documentation(DocumentationNode { string, .. }) = &mut node.kind
            && let Some((left, right)) = string.split_once("```highlight")
            && left.chars().all(char::is_whitespace)
            && !right.contains('`')
        {
            // This erases "highlight".
            *string = format!("{left}```{right}");

            nodes.insert(
                index,
                Node {
                    kind: NodeKind::Argument(ArgumentNode {
                        kind: ArgumentKind::Highlight,
                        span,
                    }),
                    style,
                },
            );

            // Increment past the inserted highlight node and the documentation node that
            // was just processed.
            index += 2;
        } else {
            index += 1;
        }
    }

    let mut delete_indices = Vec::new();
    let mut index = 0;

    while index < nodes.len() {
        let node = &nodes[index];

        let style = node.style;
        let span = node.span();

        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Highlight,
            ..
        }) = node.kind
        else {
            index += 1;
            continue;
        };

        delete_indices.push(index);

        index += 1;

        // Find the start of the code block.
        let string = loop {
            // Validate there is still a node.
            let Some(node) = nodes.get(index) else {
                return Err(Error::new(span, "expected a code block to highlight"));
            };

            if let NodeKind::Documentation(DocumentationNode { string, .. }) = &node.kind {
                break string;
            }

            index += 1;
        };

        // Trim leading whitespace characters.
        let trimmed = string.trim_start();

        // Validate the documentation starts with backticks.
        if !trimmed.starts_with("```") {
            return Err(Error::new(span, "expected a code block to highlight"));
        };

        // Validate there are only one set of backticks in the string. If there are
        // more, the code block that the highlight node is referring to is a single-line
        // code block, which isn't supported.
        if let Some(rest) = trimmed.get(3..)
            && rest.contains("```")
        {
            return Err(Error::new(
                span,
                "single-line code blocks are not supported",
            ));
        }

        index += 1;

        nodes.insert(
            index,
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: String::from("# {} /*"),
                    span,
                }),
                style,
            },
        );

        // Find the end of the code block. If there are no more nodes, the end of the
        // code block is the end of the documentation.
        while let Some(node) = nodes.get(index) {
            // If the node is a documentation node, trim leading whitespace characters and
            // check if it starts with backticks.
            if let NodeKind::Documentation(DocumentationNode { string, .. }) = &node.kind
                && string.trim_start().starts_with("```")
            {
                break;
            }

            index += 1;
        }

        nodes.insert(
            index,
            Node {
                kind: NodeKind::Documentation(DocumentationNode {
                    string: String::from("# */"),
                    span,
                }),
                style,
            },
        );

        index += 1;
    }

    // Delete the highlight nodes that were just resolved.
    delete_indices.into_iter().rev().for_each(|index| {
        nodes.remove(index);
    });

    Ok(())
}
