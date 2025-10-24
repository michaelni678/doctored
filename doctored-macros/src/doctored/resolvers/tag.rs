use syn::Result;

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

const DEFAULT_TAG_COLOR: &str = "steelblue";

pub fn resolve_tag(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Tag { text, href, color },
            ..
        }) = &node.kind
        else {
            index += 1;
            continue;
        };

        let style = node.style;
        let span = node.span();

        let color = color
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_TAG_COLOR));
        let href = href.clone().unwrap_or_default();

        let mut string = String::new();

        string.extend([
            String::from(
                r#"
<script>
    function createTag(text, color, href) {
        const heading = document.body.querySelector(".main-heading h1");

        const tagContainer = document.createElement("div");
        tagContainer.className = "doctored-tag-container";

        let tag;

        if (href) {
            tag = document.createElement("a");
            tag.setAttribute("href", href);
        } else {
            tag = document.createElement("span");
        }

        tag.className = "doctored-tag";
        tag.innerText = text;
        tag.style.backgroundColor = color;

        tagContainer.appendChild(tag);
        heading.appendChild(tagContainer);
    }
            "#,
            ),
            format! {r#"
    createTag("{text}", "{color}", "{href}");
</script>
            "#},
            String::from(
                r#"
<style>
    .doctored-tag-container {
        padding: 0.5rem 0;
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
    }
    
    .doctored-tag {
        display: flex;
        align-items: center;
        width: fit-content;
        height: 1.5rem;
        padding: 0 0.5rem;
        border-radius: 0.75rem;
        font-size: 1rem;
        font-weight: normal;
        color: white;
    }
</style>    
            "#,
            ),
        ]);

        nodes.push(Node {
            kind: NodeKind::Documentation(DocumentationNode { string, span }),
            style,
        });

        nodes[index].resolve();

        break;
    }

    Ok(())
}
