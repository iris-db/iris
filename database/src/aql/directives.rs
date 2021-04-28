use crate::aql::context::HttpContext;
use crate::aql::directive::{Directive, DirectiveError, DirectiveResult};
use crate::graph::node::CreateNodeData;

/// Insert a node into a graph.
pub struct InsertDirective;

impl Directive for InsertDirective {
  fn key(&self) -> &str {
    "insert"
  }

  fn exec(&self, ctx: HttpContext) -> DirectiveResult {
    let HttpContext {
      graph,
      data,
      refs: _,
    } = ctx;

    let nodes = data.dispatch::<CreateNodeData>(|o| {
      let data = o.get_required("data")?;

      Ok(CreateNodeData(
        Some(serde_json::to_string(data).unwrap()),
        None,
      ))
    })?;

    let res = graph.insert_nodes(Some(nodes));

    return match res {
      Ok(v) => Ok(v.0.into()),
      Err(e) => Err(DirectiveError::Serialization(e)),
    };
  }
}
