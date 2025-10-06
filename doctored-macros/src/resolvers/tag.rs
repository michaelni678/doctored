use indoc::formatdoc;
use syn::Result;

use crate::utilities::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

const DEFAULT_TAG_COLOR: &'static str = "#4470AD";

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

        let span = node.span();
        let style = node.style;

        let color = color.unwrap_or_else(|| String::from(DEFAULT_TAG_COLOR));

        let mut html_script = String::new();

        html_script.extend([
            formatdoc! {r#"
                const heading = document.body.querySelector(".main-heading h1");

                const tagContainer = document.createElement("div");
                tagContainer.className = "doctored-tag-container";

                heading.appendChild(tagContainer);
            "#},
            // If a hyperlink is provided, use an anchor. Otherwise, use a span so the text doesn't
            // get underlined when the cursor hovers over it.
            if let Some(href) = href {
                formatdoc! {r#"
                    const tag = document.createElement("a");
                    tag.setAttribute("href", "{href}");
                "#}
            } else {
                format!(r#"const tag = document.createElement("span");"#)
            },
            formatdoc! {r#"
                tag.innerText = "{text}";
                tag.className = "doctored-tag";

                tagContainer.appendChild(tag);
            "#},
        ]);

        let html_style = formatdoc! {r#"
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
        "#};

        let string = formatdoc! {r#"
            <script>{html_script}</script>
            <style>{html_style}</style>
        "#};

        nodes.push(Node {
            kind: NodeKind::Documentation(DocumentationNode { string, span }),
            style,
        });

        nodes[index].resolve();

        break;
    }

    Ok(())
}
