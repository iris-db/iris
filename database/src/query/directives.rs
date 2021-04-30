use crate::query::directive::{Directive, DirectiveResult};
use crate::server::http::context::HttpContext;

/// Insert a node into a graph.
pub struct InsertDirective;

impl Directive for InsertDirective {
  fn key(&self) -> &str {
    "insert"
  }

  fn exec(&self, ctx: HttpContext) -> DirectiveResult {
    let HttpContext { refs, graph, data } = ctx;

    let res = data.dispatch(|o| {
      let v = o.get_required("A")?;

      Ok(1)
    });
  }
}
