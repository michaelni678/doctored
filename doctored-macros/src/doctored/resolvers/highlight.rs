use syn::Result;

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

const DEFAULT_THEME: &str = "base16/atelier-dune-light.min";

pub fn resolve_highlight(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Highlight { ref theme },
            ..
        }) = node.kind
        else {
            index += 1;
            continue;
        };

        let style = node.style;
        let span = node.span();

        let theme = theme.clone().unwrap_or_else(|| String::from(DEFAULT_THEME));

        let mut string = String::new();

        string.extend([
            format! {r#"
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/{theme}.css">
            "#},
            String::from(r#"
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></script>

<script>
    document.querySelectorAll("pre").forEach(pre => {
        const code = pre.querySelector("code");

        if (!code) {
            return;
        }

        const match = pre.className.match(/language-([^\s]+)/);

        if (!match) {
            return;
        }
    
        const language = match[1];

        if (!hljs.getLanguage(language)) {
            return;
        }

        code.innerHTML = hljs.highlight(code.textContent, { language }).value;
    });
</script>
            "#),
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
