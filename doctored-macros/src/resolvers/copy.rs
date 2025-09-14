use std::collections::HashMap;

use syn::{Error, Result};

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, Node, NodeKind};

pub fn resolve_copy(
    nodes: &mut Vec<Node>,
    clipboard: &mut HashMap<String, Vec<Node>>,
) -> Result<()> {
    // Tags of heads that haven't found a matching tail yet.
    let mut copying = HashMap::new();

    // Indices of nodes to delete.
    let mut deletes = Vec::new();

    for (index, node) in nodes.iter().enumerate() {
        match node.kind {
            NodeKind::Argument(ArgumentNode { ref kind, span }) => {
                match kind {
                    ArgumentKind::CopyHead(tag) => {
                        // Validate there isn't already an unresolved head with this tag.
                        if let Some(span) = copying.get(tag).copied() {
                            return Err(Error::new(span, "copy head doesn't have a tail"));
                        }

                        copying.insert(tag.clone(), span);
                    }
                    ArgumentKind::CopyTail(tag) => {
                        // Validate there is an unresolved head with this tag.
                        if copying.remove(tag).is_none() {
                            return Err(Error::new(span, "copy tail doesn't have a head"));
                        }
                    }
                    _ => continue,
                }

                deletes.push(index);
            }
            NodeKind::Documentation(_) => {
                // For each unresolved head, push the node.
                for tag in copying.keys().cloned() {
                    clipboard.entry(tag).or_default().push(node.clone());
                }
            }
            _ => continue,
        }
    }

    deletes.reverse();

    for index in deletes {
        nodes.remove(index);
    }

    Ok(())
}
