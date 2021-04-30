use crate::query::directive::{Directive, DirectiveResult};
use crate::server::http::context::HttpContext;

/// Insert a node into a graph.
pub struct InsertDirective;

impl Directive for InsertDirective {
  fn key(&self) -> &str {
    "insert"
  }

  fn exec(&self, ctx: HttpContext) -> DirectiveResult {
    todo!()
  }
}
