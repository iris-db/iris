use crate::graph::node::Node;

/// A relationship between two Nodes. Can be one directional or bi-directional.
pub struct Edge {
    name: String,
    /// 0 - Node A
    /// 1 - Node B
    nodes: (Node, Node),
    direction: EdgeDirection,
}

/// Direction of an Edge relationship.
pub enum EdgeDirection {
    ToA,
    ToB,
    Bidirectional,
}

impl Edge {
    pub fn new(name: &str, a: Node, b: Node, direction: EdgeDirection) -> Edge {
        Edge {
            name: name.to_string(),
            nodes: (a, b),
            direction,
        }
    }

    pub fn get_nodes(&self) -> &(Node, Node) {
        &self.nodes
    }
}
