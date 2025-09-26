use proc_macro2::Span;
use syn::{AttrStyle, Attribute};

pub mod convert;

#[derive(Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub style: AttrStyle,
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
}

#[derive(Clone)]
pub struct DocumentationNode {
    pub string: String,
    pub span: Span,
}
