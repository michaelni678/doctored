use std::collections::HashMap;

use syn::Attribute;

use crate::utilities::nodes::Node;

#[derive(Default)]
pub struct Context {
    pub attrs: Vec<Attribute>,
    pub nodes: Vec<Node>,
    pub clipboard: HashMap<String, Vec<Node>>,
}
