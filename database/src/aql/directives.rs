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
      _ => return Err(DirectiveErrorType::InvalidType("Object")),
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
