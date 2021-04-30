use crate::graph::node::Node;
use crate::lib::bson::JsonObjectWrapper;
use crate::query::directive::{Directive, DirectiveResult};
use crate::server::http::context::HttpContext;
use serde_json::json;

/// Insert a node into a graph.
pub struct InsertDirective;

impl Directive for InsertDirective {
  fn key(&self) -> &str {
    "insert"
  }

  fn exec(&self, ctx: HttpContext) -> DirectiveResult {
    let HttpContext { refs, graph, data } = ctx;

    let res = data.dispatch(|o| {
      let data = o.get_optional("data");
      let data = match data {
        Some(v) => Some(v.clone()),
        None => None,
      };

      let res = graph.insert(Node::new(graph.next_id(), data, None))?;

      Ok(res as u64)
    })?;

    let res =
      JsonObjectWrapper::from(json!({ "time": res.into_iter().sum::<u64>(), "count": res.len() }));

    res.into()
  }
}
