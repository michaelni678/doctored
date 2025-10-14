use std::fs;

use syn::{
    Attribute, Error, Result, Token,
    parse::{ParseStream, Parser},
    parse_str,
};

use crate::doctored::nodes::{
    ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind,
    convert::convert_attributes_into_nodes,
};

#[derive(Clone)]
pub enum IncludeKind {
    Attributes,
    Documentation,
}

pub fn resolve_extras_include(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while index < nodes.len() {
        let node = &nodes[index];

        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Include { kind, filename },
            ..
        }) = &node.kind
        else {
            index += 1;
            continue;
        };

        let style = node.style;
        let span = node.span();

        let content = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(error) => {
                return Err(Error::new(
                    span,
                    format!("failed to read file `{filename}`: {error}"),
                ));
            }
        };

        match kind {
            IncludeKind::Attributes => {
                let attrs = Parser::parse2(
                    |input: ParseStream| {
                        if input.peek(Token![#])
                            && let Ok(attrs) = if input.peek2(Token![!]) {
                                Attribute::parse_inner(input)
                            } else {
                                Attribute::parse_outer(input)
                            }
                        {
                            Ok(attrs)
                        } else {
                            Err(Error::new(
                                span,
                                format!("expected only attributes in included file `{filename}`"),
                            ))
                        }
                    },
                    parse_str(&content)?,
                )?;

                nodes[index].resolve();
                index += 1;

                // TODO: Set the spans of the loaded nodes to the span of the load node.
                nodes.splice(index..index, convert_attributes_into_nodes(attrs)?);
            }
            IncludeKind::Documentation => {
                nodes[index].resolve();
                index += 1;

                nodes.insert(
                    index,
                    Node {
                        kind: NodeKind::Documentation(DocumentationNode {
                            string: content,
                            span,
                        }),
                        style,
                    },
                );
            }
        }
    }

    Ok(())
}
