use crate::aql::context::AqlContext;
use crate::aql::directive::{
  extract_directive_data, Directive, DirectiveDataExtraction, DirectiveErrorType, DirectiveResult,
};
use crate::graph::node::CreateNodeData;

/// Insert a node into a graph.
pub struct InsertDirective;

impl Directive for InsertDirective {
  fn key(&self) -> &str {
    "insert"
  }

  fn exec(&self, ctx: &mut AqlContext) -> DirectiveResult {
    let graph = &mut ctx.graph;

    let data = extract_directive_data(self, ctx.data);
    let data = match data {
      DirectiveDataExtraction::Array(v) => v,
      _ => return Err(DirectiveErrorType::InvalidType("array")),
    };

    let mut nodes = Vec::new();

    for o in data {
      let data_key = "data";

      let data = match o.get(data_key) {
        Some(v) => v,
        None => return Err(DirectiveErrorType::MissingKey(data_key)),
      };

      nodes.push(CreateNodeData(Some(data.to_string()), None));
    }

    let res = graph.insert_nodes(Some(nodes));
    let res = match res {
      Ok(v) => v,
      Err(e) => return Err(DirectiveErrorType::Serialization(e)),
    };

    Ok(res.0.into())
  }
}

/// Deletes a node from a graph.
pub struct DeleteDirective;

impl Directive for DeleteDirective {
  fn key(&self) -> &str {
    "delete"
  }

  fn exec(&self, ctx: &mut AqlContext) -> DirectiveResult {
    let graph = &mut ctx.graph;

    let data = extract_directive_data(self, ctx.data);
    let data = match data {
      DirectiveDataExtraction::Array(v) => v,
      _ => return Err(DirectiveErrorType::InvalidType("array")),
    };

    let t = &data[0];

    let res = graph.delete_node_by_id(t.get("id").unwrap().as_u64().unwrap());

    Ok(res.into())
  }
}
