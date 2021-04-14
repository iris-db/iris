use crate::io::bson_util::write_chunk;
use crate::lib::common::JsonObject;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::time::Instant;

pub type NodeId = u128;

/// A data node. Can be connected by edges.
///
/// The proper way to create a node is through a NodePlane's insert_node function.
#[derive(Eq)]
pub struct Node {
    pub id: NodeId,
    pub groups: Option<Vec<String>>,
    pub data: Option<JsonObject>,
}

impl Node {
    pub fn serialize(&self) -> u128 {
        let now = Instant::now();

        let data = self.data.clone();
        let data = match data {
            Some(data) => data.clone(),
            None => return 0,
        };

        let doc = bson::Document::try_from(data).unwrap();

        let mut buf = Vec::new();
        doc.to_writer(&mut buf).unwrap();

        let res = write_chunk("Data/Index.0", buf);
        match res {
            Ok(()) => now.elapsed().as_millis(),
            _ => 0,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
