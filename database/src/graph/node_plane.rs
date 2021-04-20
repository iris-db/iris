extern crate test;

use std::fs;
use std::io::Error;
use std::time::Instant;

use serde_json::{json, Value};

use crate::graph::edge::Edge;
use crate::graph::node::{Node, NodeId};
use crate::lib::bson::encode;
use crate::lib::filesystem::DATA_PATH;
use crate::lib::uid::IntCursor;

const MAX_NODE_SIZE: usize = 8000;
const MAX_PAGE_SIZE: u64 = 16000;

/// A collection of graph nodes.
pub struct NodePlane {
    /// The name of the plane.
    name: String,
    /// Next available id.
    cursor: IntCursor,
    /// Nodes present within the plane. Always sorted by id.
    nodes: Vec<Box<Node>>,
    /// Current page. A page is an 8KB file that contains serialized nodes. They are loaded into
    /// the nodes vec that tries to load as many nodes in memory as possible (generally the maximum
    /// amount of nodes that the nodes vec holds is < 32GB.
    page_pos: u32,
}

/// Result of deleting a node.
pub struct DeleteResult {
    count: u32,
    time: u128,
}

/// Error that occurs while serializing a node. Errors can be caused by the filesystem or a node
/// format error, such as exceeding the maximum node size.
pub enum SerializationError {
    /// Caused by the rust fs api.
    Filesystem(Error),
    /// Caused by a node exceeded the maximum node size.
    NodeSizeExceeded(NodeId),
}

/// Methods for marshaling errors into JSON error objects.
impl SerializationError {
    fn marshal_filesystem_error(e: Error) -> Value {
        let s = e.to_string();

        json!({
            "error": {
                "msg": format!("CRITICAL FILESYSTEM ERROR: {}", s),
                "data": s
            }
        })
    }

    fn marshal_node_size_exceeded_error(id: NodeId) -> Value {
        json!({
            "error": {
                "msg": format!("[Node Id: {}] Exceeded the maximum node size of {} bytes", id, MAX_NODE_SIZE),
                "data": id
            }
        })
    }
}

/// Public API. This includes CRUD operations.
impl NodePlane {
    /// Creates a new node plane.
    pub fn new(name: &str) -> Box<NodePlane> {
        Box::from(NodePlane {
            name: name.to_string(),
            cursor: IntCursor::new(),
            nodes: Vec::new(),
            page_pos: 0,
        })
    }

    // Inserts a node into a node plane, returning its unique id.
    pub fn insert_node(&mut self, bson: Option<&str>, edges: Option<Vec<Edge>>) -> NodeId {
        let id = self.cursor.next();

        let n = Node::new(
            id,
            bson.unwrap_or("{}").to_string(),
            edges.unwrap_or(Vec::new()),
        );

        let res = self.serialize_node();

        self.nodes.push(Box::from(n));
        self.nodes.sort();

        id
    }

    pub fn delete_node_by_id(&mut self, id: NodeId) -> DeleteResult {
        let now = Instant::now();
        let mut count = 0;

        let pos = self.nodes.binary_search_by(|node| node.id().cmp(&id));
        let pos = match pos {
            Ok(pos) => pos,
            Err(_) => {
                return DeleteResult {
                    count,
                    time: now.elapsed().as_millis(),
                }
            }
        };

        self.nodes.remove(pos);
        count += 1;

        DeleteResult {
            count,
            time: now.elapsed().as_millis(),
        }
    }

    /// Deletes a node based on a predicate.
    pub fn delete_node_where(
        &mut self,
        predicate: fn(&Node) -> bool,
        limit: Option<u32>,
    ) -> DeleteResult {
        let mut count = 0;
        let nodes = &mut self.nodes;

        let limit_exists = limit.is_some();

        let now = Instant::now();

        for i in 0..nodes.len() {
            let n = nodes.get(i).unwrap();

            if predicate(n) {
                nodes.remove(i);
                count += 1;
            }

            if limit_exists && count == limit.unwrap() {
                break;
            }
        }

        DeleteResult {
            count,
            time: now.elapsed().as_millis(),
        }
    }
}

/// Serialization API. Methods for serializing data to the disk.
impl NodePlane {
    /// Serializes a node onto the filesystem.
    ///
    /// Returns the serialization time if successful.
    fn serialize_node(&mut self, node: &Node) -> Result<u128, SerializationError> {
        let time = Instant::now();

        let bytes = encode(node.bson());

        let mut path = self.current_data_path();
        let metadata = fs::metadata(path.clone());
        let metadata = match metadata {
            Ok(v) => v,
            Err(e) => return Err(SerializationError::Filesystem(e)),
        };

        if bytes.len() > MAX_NODE_SIZE {
            return Err(SerializationError::NodeSizeExceeded(*node.id()));
        }

        if metadata.len() > MAX_PAGE_SIZE {
            self.page_pos += 1;
            path = self.current_data_path();
        }

        let res = fs::write(path, bytes);
        match res {
            Err(e) => return Err(SerializationError::Filesystem(e)),
            _ => {}
        }

        Ok(time.elapsed().as_millis())
    }

    /// File location of the active page.
    fn current_data_path(&self) -> String {
        format!("{}/{}.{}", DATA_PATH, self.name, self.page_pos)
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn it_inserts_into_a_node_plane() {
        let mut p = NodePlane::new("TEST");
        let id = p.insert_node(None, None);

        assert_eq!(0, id);
    }

    #[test]
    fn it_deletes_a_node_by_id() {
        let mut p = NodePlane::new("TEST");

        let node_a = p.insert_node(None, None);
        let node_b = p.insert_node(None, None);
        let node_c = p.insert_node(None, None);

        let res_a = p.delete_node_by_id(node_a);
        assert_eq!(res_a.count, 1);

        let res_c = p.delete_node_by_id(node_c);
        assert_eq!(res_c.count, 1);

        let res_b = p.delete_node_by_id(node_b);
        assert_eq!(res_b.count, 1);

        let res_err = p.delete_node_by_id(4);
        assert_eq!(res_err.count, 0);
    }

    #[bench]
    fn bench_insert_nodes(b: &mut Bencher) {
        let mut p = NodePlane::new("TEST");

        b.iter(|| p.insert_node(Some("{\n  \"created_at\": \"Thu Jun 22 21:00:00 +0000 2017\",\n  \"id\": 877994604561387500,\n  \"id_str\": \"877994604561387520\",\n  \"text\": \"Creating a Grocery List Manager Using Angular, Part 1: Add &amp; Display Items https://t.co/xFox78juL1 #Angular\",\n  \"truncated\": false,\n  \"entities\": {\n    \"hashtags\": [{\n      \"text\": \"Angular\",\n      \"indices\": [103, 111]\n    }],\n    \"symbols\": [],\n    \"user_mentions\": [],\n    \"urls\": [{\n      \"url\": \"https://t.co/xFox78juL1\",\n      \"expanded_url\": \"http://buff.ly/2sr60pf\",\n      \"display_url\": \"buff.ly/2sr60pf\",\n      \"indices\": [79, 102]\n    }]\n  },\n  \"source\": \"<a href=\\\"http://bufferapp.com\\\" rel=\\\"nofollow\\\">Buffer</a>\",\n  \"user\": {\n    \"id\": 772682964,\n    \"id_str\": \"772682964\",\n    \"name\": \"SitePoint JavaScript\",\n    \"screen_name\": \"SitePointJS\",\n    \"location\": \"Melbourne, Australia\",\n    \"description\": \"Keep up with JavaScript tutorials, tips, tricks and articles at SitePoint.\",\n    \"url\": \"http://t.co/cCH13gqeUK\",\n    \"entities\": {\n      \"url\": {\n        \"urls\": [{\n          \"url\": \"http://t.co/cCH13gqeUK\",\n          \"expanded_url\": \"https://www.sitepoint.com/javascript\",\n          \"display_url\": \"sitepoint.com/javascript\",\n          \"indices\": [0, 22]\n        }]\n      },\n      \"description\": {\n        \"urls\": []\n      }\n    },\n    \"protected\": false,\n    \"followers_count\": 2145,\n    \"friends_count\": 18,\n    \"listed_count\": 328,\n    \"created_at\": \"Wed Aug 22 02:06:33 +0000 2012\",\n    \"favourites_count\": 57,\n    \"utc_offset\": 43200,\n    \"time_zone\": \"Wellington\"\n  }"), None));
    }
}
