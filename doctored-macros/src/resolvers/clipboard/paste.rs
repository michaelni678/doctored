use syn::{Error, Result};

use crate::{
    resolvers::clipboard::apply_clipboard_modifiers,
    utilities::{
        context::Context,
        nodes::{ArgumentKind, ArgumentNode, DocumentationNode, NodeKind},
    },
};

pub fn resolve_clipboard_paste(context: &mut Context) -> Result<()> {
    let mut resolved_indices = Vec::new();
    let mut index = 0;

    while index < context.nodes.len() {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::ClipboardPaste { name, modifiers },
            span,
            ..
        }) = &context.nodes[index].kind
        else {
            index += 1;
            continue;
        };

        let Some(mut content) = context.clipboard.get(name).cloned() else {
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
        context.nodes.splice(index..index, content);
    }

    for index in resolved_indices {
        context.nodes[index].resolve();
    }

    Ok(())
}
