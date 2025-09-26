use syn::{Attribute, Result};

use crate::utilities::nodes::Node;

pub fn convert_attributes_into_nodes(attrs: Vec<Attribute>) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();

    Ok(nodes)
}

pub fn convert_nodes_into_attributes(nodes: Vec<Node>) -> Result<Vec<Attribute>> {
    let mut attrs = Vec::new();

    Ok(attrs)
}
