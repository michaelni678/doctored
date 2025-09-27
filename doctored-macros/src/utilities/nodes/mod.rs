use proc_macro2::Span;
use syn::{AttrStyle, Attribute, spanned::Spanned};

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
}

#[derive(Clone)]
pub enum ArgumentKind {
    SummaryHide,
    SummaryMock(String),
    Highlight,
}

#[derive(Clone)]
pub struct DocumentationNode {
    pub string: String,
    pub span: Span,
}
