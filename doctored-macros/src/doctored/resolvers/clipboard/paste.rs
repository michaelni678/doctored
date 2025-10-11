use std::collections::HashMap;

use syn::{Error, Result};

use crate::doctored::{
    nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind},
    resolvers::clipboard::apply_clipboard_modifiers,
};

pub fn resolve_clipboard_paste(
    nodes: &mut Vec<Node>,
    clipboard: HashMap<String, Vec<Node>>,
) -> Result<()> {
    let mut resolved_indices = Vec::new();
    let mut index = 0;

    while index < nodes.len() {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::ClipboardPaste { name, modifiers },
            span,
            ..
        }) = &nodes[index].kind
        else {
            index += 1;
            continue;
        };

        let Some(mut content) = clipboard.get(name).cloned() else {
            return Err(Error::new(*span, "paste has no content"));
        };

        for node in content.iter_mut() {
            let NodeKind::Documentation(DocumentationNode { string, .. }) = &mut node.kind else {
                unreachable!();
            };

            apply_clipboard_modifiers(modifiers, string);
        }

        resolved_indices.push(index);
        index += 1;

        // Insert all the copied content behind the paste node.
        nodes.splice(index..index, content);
    }

    for index in resolved_indices {
        nodes[index].resolve();
    }

    Ok(())
}
