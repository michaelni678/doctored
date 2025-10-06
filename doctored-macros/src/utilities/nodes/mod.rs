use proc_macro2::Span;
use syn::{AttrStyle, Attribute, spanned::Spanned};

use crate::resolvers::clipboard::ClipboardModifier;

pub mod convert;

#[derive(Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub style: AttrStyle,
}

impl Node {
    pub fn span(&self) -> Span {
        match self.kind {
            NodeKind::Argument(ArgumentNode { span, .. })
            | NodeKind::Documentation(DocumentationNode { span, .. }) => span,
            NodeKind::Unrelated(ref attr) => attr.span(),
        }
    }

    /// Resolves the node.
    pub fn resolve(&mut self) {
        let NodeKind::Argument(ArgumentNode { resolved, .. }) = &mut self.kind else {
            panic!("expected an argument node");
        };

        *resolved = true;
    }
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum NodeKind {
    Argument(ArgumentNode),
    Documentation(DocumentationNode),
    Unrelated(Attribute),
}

#[derive(Clone)]
pub struct ArgumentNode {
    pub kind: ArgumentKind,
    pub span: Span,
    pub resolved: bool,
}

#[derive(Clone)]
pub enum ArgumentKind {
    SummaryHide,
    SummaryMock {
        summary: String,
    },
    Highlight,
    ClipboardCopyHead {
        name: String,
        modifiers: Vec<ClipboardModifier>,
    },
    ClipboardCopyTail {
        name: String,
    },
    ClipboardPaste {
        name: String,
        modifiers: Vec<ClipboardModifier>,
    },
    Tag {
        text: String,
        href: Option<String>,
        text_color: Option<String>,
        background_color: Option<String>,
    },
}

#[derive(Clone)]
pub struct DocumentationNode {
    pub string: String,
    pub span: Span,
}
