use std::collections::HashMap;

use crate::utilities::nodes::Node;

#[derive(Default)]
pub struct Context {
    pub nodes: Vec<Node>,
    pub clipboard: HashMap<String, Vec<Node>>,
}
