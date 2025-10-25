use syn::Result;

use crate::doctored::nodes::{ArgumentKind, ArgumentNode, DocumentationNode, Node, NodeKind};

pub fn resolve_highlight(nodes: &mut Vec<Node>) -> Result<()> {
    let mut index = 0;

    while let Some(node) = nodes.get(index) {
        let NodeKind::Argument(ArgumentNode {
            kind: ArgumentKind::Highlight,
            ..
        }) = node.kind
        else {
            index += 1;
            continue;
        };

        let style = node.style;
        let span = node.span();

        let string = String::from(
            r#"
<style>
    .hljs {
        display: block;
        overflow-x: auto;
        padding: 0.5em;
    }

    .hljs-comment,
    .hljs-quote {
        color: var(--code-highlight-comment-color);
    }

    .hljs-number {
        color: var(--code-highlight-number-color);
    }

    .hljs-literal {
        color: var(--code-highlight-literal-color);
    }

    .hljs-type {
        color: var(--code-highlight-kw-2-color);
    }

    .hljs-string,
    .hljs-symbol,
    .hljs-bullet {
        color: var(--code-highlight-string-color);
    }

    .hljs-section {
        color: var(--code-highlight-macro-color);
    }

    .hljs-built_in,
    .hljs-attr,
    .hljs-name,
    .hljs-tag {
        color: var(--code-highlight-self-color);
    }

    .hljs-keyword,
    .hljs-selector-tag {
        color: var(--code-highlight-kw-color);
    }

    .hljs-emphasis {
        font-style: italic;
    }

    .hljs-strong {
        font-weight: bold;
    }

    .hljs-addition {
        color: #44be60ff;
        background-color: var(--stab-background-color);
    }

    .hljs-deletion {
        color: #dd424cff;
        background-color: var(--stab-background-color);
    }
</style>

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
        "#,
        );

        nodes.push(Node {
            kind: NodeKind::Documentation(DocumentationNode { string, span }),
            style,
        });

        nodes[index].resolve();

        break;
    }

    Ok(())
}
