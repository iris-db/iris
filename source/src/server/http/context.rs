use std::collections::HashMap;

use serde_json::Value;

use crate::graph::graph::Graph;
use crate::lib::bson::{values_to_objects, JsonObject};
//
// /// Holds the refs and other metadata about the request.
// pub struct HttpContext<'a> {
//   pub graph: &'a mut Graph,
//   pub data: DirectiveDataSet,
//   pub refs: HashMap<String, JsonObject>,
// }
//
// impl HttpContext<'_> {
//   pub fn try_new<'a>(
//     graph: &'a mut Graph,
//     body: &JsonObject,
//   ) -> Result<HttpContext<'a>, DirectiveError> {
//     let data = HttpContext::lookup_directive_data(body)?;
//
//     Ok(HttpContext {
//       graph,
//       data,
//       refs: HttpContext::traverse_refs(body),
//     })
//   }
// }
