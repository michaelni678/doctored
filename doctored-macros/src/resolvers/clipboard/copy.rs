use std::collections::HashMap;

use crate::{
    resolvers::clipboard::apply_clipboard_modifiers,
    utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind},
};

pub fn resolve_clipboard_copy(nodes: &mut Vec<Node>, clipboard: &mut HashMap<String, Vec<Node>>) {
    // The tags currently being copied and their starting node index.
    let mut copying = HashMap::new();

    let mut resolved_indices = Vec::new();

    for (index, node) in nodes.iter().enumerate() {
        match &node.kind {
            NodeKind::Argument(ArgumentNode {
                kind: ArgumentKind::ClipboardCopy { tag, modifiers },
                ..
            }) => {
                if let Some((starting_node_index, _)) = copying.remove(tag) {
                    // The starting and ending copy nodes can now be resolved.
                    resolved_indices.push(starting_node_index);
                    resolved_indices.push(index);
                } else {
                    copying.insert(tag.clone(), (index, modifiers.clone()));
                }
            }
            NodeKind::Documentation(_) => {
                for (tag, (_, modifiers)) in copying.iter() {
                    let mut node = node.clone();

                    let NodeKind::Documentation(DocumentationNode { string, .. }) = &mut node.kind
                    else {
                        unreachable!();
                    };

                    apply_clipboard_modifiers(modifiers, string);

                    clipboard.entry(tag.clone()).or_default().push(node);
                }
            }
            _ => continue,
        }
    }

    for index in resolved_indices {
        nodes[index].resolve();
    }
}
