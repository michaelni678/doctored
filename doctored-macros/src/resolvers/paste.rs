use std::collections::HashMap;

use syn::{Error, Result};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn resolve_paste(nodes: &mut Vec<Node>, clipboard: &HashMap<String, Vec<Node>>) -> Result<()> {
    let mut inserts = Vec::new();

    for (index, node) in nodes.iter().enumerate() {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Paste(ref tag),
            span,
        }) = node.kind
        else {
            continue;
        };

        inserts.push((index, tag.clone(), span));
    }

    inserts.reverse();

    for (index, tag, span) in inserts {
        let Some(content) = clipboard.get(&tag) else {
            return Err(Error::new(span, "paste has no content"));
        };

        // Replace the node at the index with the paste content.
        nodes.splice(index..=index, content.clone());
    }

    Ok(())
}
