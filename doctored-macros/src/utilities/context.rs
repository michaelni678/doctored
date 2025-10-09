use std::collections::HashMap;

use syn::Attribute;

use crate::utilities::nodes::Node;

#[derive(Default)]
pub struct Context {
    /// The original attributes.
    pub attrs: Vec<Attribute>,
    /// The nodes, which are created from the original attributes `attrs`.
    pub nodes: Vec<Node>,
    pub clipboard: HashMap<String, Vec<Node>>,
}
