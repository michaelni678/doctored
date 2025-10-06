use indoc::formatdoc;
use syn::Result;

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_tag(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind:
                ArgumentKind::Tag {
                    text,
                    href,
                    text_color,
                    background_color,
                },
            ..
        }) = node.kind.clone()
        else {
            index += 1;
            continue;
        };

        let span = node.span();
        let style = node.style;

        let href = href.map_or_else(String::new, |href| {
            format!(r#"tag.setAttribute("href", "{href}");"#)
        });
        let text_color = text_color.unwrap_or_else(|| String::from("white"));
        let background_color =
            background_color.unwrap_or_else(|| String::from("oklch(50% 27% 110)"));

        nodes.push(Node {
            kind: NodeKind::Documentation(DocumentationNode {
                string: String::from(formatdoc! {r#"
                        <script>
                            const heading = document.body.querySelector(".main-heading h1");

                            const tagContainer = document.createElement("div");
                            tagContainer.className = "doctored-tag-container";

                            heading.appendChild(tagContainer);
                            
                            const tag = document.createElement("a");
                            tag.innerText = "{text}";
                            {href}
                            tag.className = "doctored-tag";

                            tagContainer.appendChild(tag);
                        </script>

                        <style>
                            .doctored-tag-container {{
                                padding: 0.5rem 0;
                                display: flex;
                                flex-wrap: wrap;
                                gap: 0.5rem;
                            }}
                            
                            .doctored-tag {{
                                display: flex;
                                align-items: center;
                                width: fit-content;
                                height: 1.5rem;
                                padding: 0 0.5rem;
                                border-radius: 0.75rem;
                                font-size: 1rem;
                                font-weight: normal;
                                color: {text_color};
                                background-color: {background_color};
                            }}
                        </style>
                    "#}),
                span,
            }),
            style,
        });

        nodes[index].resolve();

        break;
    }

    Ok(())
}
