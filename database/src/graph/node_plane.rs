extern crate test;

use crate::graph::edge::Edge;
use crate::graph::node::{Node, NodeId};
use crate::lib::uid::IntCursor;

/// A collection of graph nodes.
pub struct NodePlane {
    pub cursor: IntCursor,
    pub nodes: Vec<Box<Node>>,
}

impl NodePlane {
    /// Creates a new node plane.
    pub fn new() -> Box<NodePlane> {
        Box::from(NodePlane {
            cursor: IntCursor::new(),
            nodes: Vec::new(),
        })
    }

    // Inserts a node into a node plane, returning its unique id.
    fn new_node(&mut self, bson: Option<&str>, edges: Option<Vec<Edge>>) -> NodeId {
        let id = self.cursor.next();

        let n = Node {
            id,
            bson: bson.unwrap_or("").to_string(),
            edges: edges.unwrap_or(Vec::new()),
        };

        self.nodes.push(Box::from(n));

        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_inserts_into_a_node_plane() {
        let mut p = NodePlane::new();
        let id = p.new_node(None, None);

        assert_eq!(0, id);
    }

    #[bench]
    fn bench_insert_nodes(b: &mut Bencher) {
        let mut p = NodePlane::new();

        b.iter(|| p.new_node(Some(""), None));
    }
}
