use crate::graph::graph::Graph;
use crate::graph::node::Node;
use crate::lib::bson::JsonObject;
use crate::query::query::{Query, QueryResult};
use crate::server::http::smart_json::SmartJsonObject;
use serde::{Deserialize, Serialize};

/// Insert a node into a graph.
pub struct Insert;

/// The amount of time the insert took.
pub type InsertResult = u64;

impl Query<InsertResult> for Insert {
  fn exec(graph: &mut Graph, data: &JsonObject) -> QueryResult<InsertResult> {
    // let data = SmartJsonObject(data);
    //
    // let group = data.get_owned("group");
    //
    // let group = data.get("group");
    // let group = match group {
    //   Some(value) => Some(value.clone()),
    //   None => None,
    // };
    //
    // let node_data = data.get("data");
    // let node_data = match node_data {
    //   Some(value) => Some(value.clone()),
    //   None => None,
    // };
    //
    // let id = graph.next_id();
    //
    // let time = graph.insert(Node::new(id, group, node_data, None))?;

    Ok(0 as u64)
  }
}
