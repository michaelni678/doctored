use syn::Result;

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

const DEFAULT_TAG_COLOR: &str = "#4470AD";

pub fn resolve_tag(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Tag { text, href, color },
            ..
        }) = node.kind.clone()
        else {
            index += 1;
            continue;
        };

        let attr_style = node.attr_style;
        let span = node.span();

        let color = color.unwrap_or_else(|| String::from(DEFAULT_TAG_COLOR));

        let mut string = String::new();

        string.extend([
            String::from(
                r#"
<script>
    const heading = document.body.querySelector(".main-heading h1");

    const tagContainer = document.createElement("div");
    tagContainer.className = "doctored-tag-container";
            "#,
            ),
            if let Some(href) = href {
                format! {r#"
    const tag = document.createElement("a");
    tag.setAttribute("href", "{href}");
                "#}
            } else {
                String::from(
                    r#"
    const tag = document.createElement("span");
                "#,
                )
            },
            format! {r#"
    tag.className = "doctored-tag";
    tag.innerText = "{text}";

    tagContainer.appendChild(tag);
    heading.appendChild(tagContainer);
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
        color: white;
        background-color: {color};
    }}
</style>
            "#},
        ]);

        nodes.push(Node {
            kind: NodeKind::Documentation(DocumentationNode { string, span }),
            attr_style,
        });

        nodes[index].resolve();

        break;
    }

    Ok(())
}
