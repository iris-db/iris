use crate::aql::context::AqlContext;
use crate::aql::directive::Directive;
use crate::lib::bson::JsonObject;

/// Insert a document into a graph.
pub struct InsertDirective {}

impl Directive for InsertDirective {
    fn key(&self) -> &str {
        "insert"
    }

    fn exec(&self, ctx: &AqlContext) -> JsonObject {
        let x = ctx.extract_directive_data(self);

        let plane = ctx.plane();

        serde_json::json!({}).as_object().unwrap().clone()
    }
}
