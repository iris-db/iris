use crate::graph::node::{Node, NodeId};
use crate::lib::common::JsonObject;
use crate::lib::uid::IntCursor;
use std::collections::{BTreeMap, BTreeSet};

/// A collection of Nodes.
pub struct NodePlane {
    name: String,
    nodes: BTreeSet<Box<Node>>,
    cursor: IntCursor,
}

impl NodePlane {
    /// Creates a new node plane from a name.
    pub fn new(name: &str) -> NodePlane {
        NodePlane {
            name: name.to_string(),
            nodes: BTreeSet::new(),
            cursor: IntCursor::new(),
        }
    }

    /// Inserts a new node into the node tree, returning its uid.
    pub fn insert_node(
        &mut self,
        groups: Option<Vec<String>>,
        data: Option<JsonObject>,
    ) -> &Box<Node> {
        let id = self.cursor.next();
        let n = Box::from(Node { id, groups, data });

        n.serialize();

        self.nodes.insert(id, n);
        self.nodes.get(&id).unwrap()
    }

    pub fn get_node_by_id(&self, id: NodeId) -> Option<&Box<Node>> {
        let v: Vec<Box<Node>> = self.nodes.iter().collect();

        self.nodes.self.nodes.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_inserts_nodes_by_index() {
        let mut p = NodePlane::new("myn");

        let n = p.insert_node(
            None,
            Some(
                serde_json::json!({
                    "hello": "world"
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        );

        let nk = p.get_node_by_id(0).unwrap();
        println!("{:?}", nk.data);
    }
}
