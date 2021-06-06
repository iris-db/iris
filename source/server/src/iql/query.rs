use crate::storage_engines::affinity::graph::Graph;
use serde_json::Value;

pub struct Query<'a> {
    ctx: ExecutionContext<'a>,
    stmts: Vec<&'a Value>,
}

pub struct ExecutionContext<'a> {
    graph: &'a mut Graph,
}

impl<'a> Query<'a> {
    fn new(ctx: ExecutionContext<'a>, stmts: Vec<&'a Value>) -> Self {
        Query { ctx, stmts }
    }

    fn dispatch(&self) -> () {
        for val in &self.stmts {
            let a = &val[0];
        }
    }
}
