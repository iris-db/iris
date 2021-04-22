use crate::aql::directive::Directive;
use crate::graph::node_plane::NodePlane;
use crate::lib::bson::JsonObject;

pub struct InsertDirective {}

impl Directive for InsertDirective {
    fn key(&self) -> &str {
        "insert"
    }

    fn exec(&self, plane: &NodePlane) -> JsonObject {
        println!("Exec_stat");
        serde_json::json!({}).as_object().unwrap().clone()
    }
}
