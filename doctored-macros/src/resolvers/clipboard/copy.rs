use std::collections::HashMap;

use proc_macro2::Span;
use syn::{Error, Result};

use crate::{
    resolvers::clipboard::{ClipboardModifier, apply_clipboard_modifiers},
    utilities::{
        context::Context,
        nodes::{ArgumentKind, ArgumentNode, DocumentationNode, NodeKind},
    },
};

struct Head<'a> {
    index: usize,
    span: Span,
    modifiers: &'a [ClipboardModifier],
}

pub fn resolve_clipboard_copy(context: &mut Context) -> Result<()> {
    // The heads that haven't found a matching tail.
    let mut heads: HashMap<String, Head> = HashMap::new();

    let mut resolved_indices = Vec::new();

    for (index, node) in context.nodes.iter().enumerate() {
        match node.kind {
            NodeKind::Argument(ArgumentNode {
                kind:
                    ArgumentKind::ClipboardCopyHead {
                        ref name,
                        ref modifiers,
                    },
                span,
                ..
            }) => {
                if let Some(head) = heads.get(name) {
                    return Err(Error::new(head.span, "no tail found"));
                }

                heads.insert(
                    name.clone(),
                    Head {
                        index,
                        span,
                        modifiers,
                    },
                );
            }
            NodeKind::Argument(ArgumentNode {
                kind: ArgumentKind::ClipboardCopyTail { ref name },
                span,
                ..
            }) => {
                let Some(head) = heads.remove(name) else {
                    return Err(Error::new(span, "no head found"));
                };

                // The head and tail nodes can be resolved now.
                resolved_indices.extend([head.index, index]);
            }
            NodeKind::Documentation(_) => {
                for (name, head) in heads.iter() {
                    let mut node = node.clone();

                    let NodeKind::Documentation(DocumentationNode { string, .. }) = &mut node.kind
                    else {
                        unreachable!();
                    };

                    apply_clipboard_modifiers(head.modifiers, string);

                    context
                        .clipboard
                        .entry(name.clone())
                        .or_default()
                        .push(node);
                }
            }
            _ => continue,
        }
    }

    // Validate there are no unresolved heads.
    if let Some(head) = heads.values().next() {
        return Err(Error::new(head.span, "no tail found"));
    }

    for index in resolved_indices {
        context.nodes[index].resolve();
    }

    Ok(())
}
