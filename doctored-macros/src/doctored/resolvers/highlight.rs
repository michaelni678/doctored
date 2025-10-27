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

    .hljs-keyword,
    .hljs-section {
        color: var(--code-highlight-kw-color);
    }

    .hljs-type {
        color: var(--code-highlight-kw-2-color);
    }

    .hljs-string,
    .hlhs-regexp {
        color: var(--code-highlight-string-color);
    }

    .hljs-number {
        color: var(--code-highlight-number-color);
    }

    .hljs-literal {
        color: var(--code-highlight-literal-color);
    }

    .hljs-subst {
        color: color-mix(
            in srgb,
            var(--code-highlight-string-color) 30%,
            var(--code-highlight-macro-color) 70%
        );
    }

    .hljs-name,
    .hljs-tag,
    .hljs-attr,
    .hljs-attribute {
        color: var(--code-highlight-self-color);
    }

    .hljs-built_in {
        color: var(--code-highlight-prelude-color);
    }

    .hljs-symbol,
    .hljs-bullet {
        color: var(--code-highlight-question-mark-color);  
    }

    .hljs-comment {
        color: var(--code-highlight-comment-color);
    }

    .hljs-quote,
    .hljs-doctag {
        color: var(--code-highlight-doc-comment-color);
    }

    .hljs-addition {
        color: color-mix(in oklab, var(--main-color) 40%, limegreen);
        background-color: var(--main-background-color);
    }

    .hljs-deletion {
        color: color-mix(in oklab, var(--main-color) 40%, crimson);
        background-color: var(--main-background-color);
    }

    .hljs-emphasis {
        font-style: italic;
    }

    .hljs-strong {
        font-weight: bold;
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
