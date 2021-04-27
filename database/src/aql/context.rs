use std::collections::HashMap;

use crate::graph::graph::Graph;
use crate::lib::bson::JsonObject;

/// The JSON key that denotes a reference.
const REF_KEY: &str = "$ref";

/// Holds the refs and other metadata about the request.
pub struct AqlContext<'a> {
  /// Current node graph.
  pub graph: &'a mut Graph,
  /// JSON object that contains the directive data. It is an array.
  pub data: &'a JsonObject,
  /// JSON object references in the request body.
  pub refs: HashMap<String, JsonObject>,
}

impl AqlContext<'_> {
  pub fn new<'a>(graph: &'a mut Graph, body: &'a JsonObject) -> AqlContext<'a> {
    AqlContext {
      graph,
      data: body,
      refs: AqlContext::traverse_refs(body),
    }
  }

  /// Traverses each JSON object for a reference.
  fn traverse_refs(json: &JsonObject) -> HashMap<String, JsonObject> {
    fn traverse_object(tree: &mut HashMap<String, JsonObject>, ch: &JsonObject) {
      for (k, v) in ch {
        if k == REF_KEY && v.is_string() {
          tree.insert(v.as_str().unwrap().to_string(), ch.clone());
        }

        if v.is_object() {
          traverse_object(tree, v.as_object().unwrap());
          return;
        }

        if v.is_array() {
          for v in v.as_array().unwrap() {
            traverse_object(tree, v.as_object().unwrap());
          }
        }
      }
    }

    let mut tree = HashMap::new();

    traverse_object(&mut tree, json);

    tree
  }
}

#[cfg(test)]
mod tests {
  use serde_json::json;

  use crate::aql::directive::{
    extract_directive_data, Directive, DirectiveDataExtraction, DirectiveResult,
  };
  use crate::lib::bson::Json;

  use super::*;

  #[test]
  fn test_extract_directive_data() {
    let json = Json::from(json!(
      {
        "insert": [
          {
            "$ref": "c",
            "data": {
              "age": 32,
              "height": "50cm",
              "settings": {
                "theme": "dark"
              }
            }
          }
        ]
      }
    ));

    struct TestDirective {}

    impl Directive for TestDirective {
      fn key(&self) -> &str {
        "insert"
      }

      fn exec(&self, _ctx: &mut AqlContext) -> DirectiveResult {
        todo!()
      }
    }

    let directive = &TestDirective {};
    let data = extract_directive_data(directive, json.to_object_ref());

    let data = match data {
      DirectiveDataExtraction::Array(v) => v,
      _ => panic!("Expected an object"),
    };

    assert!(
      data[0].eq(
        Json::from(json!({
            "$ref": "c",
            "data": {
              "age": 32,
              "height": "50cm",
              "settings": {
                "theme": "dark"
              }
            }
        }))
        .to_object_ref()
      )
    );
  }

  #[test]
  fn test_ref_traversal() {
    let g = &mut Graph::new("TEST");

    let json = Json::from(json!(
      {
        "$get": [
          {
            "$ref": "a",
            "a": "b"
          },
          {
            "$ref": "b",
            "username": "Steve",
            "name": {
              "first": "John",
              "last": "Smith"
            }
          },
        ],
        "$insert": [
          {
            "$ref": "c",
            "data": {
              "age": 32,
              "height": "50cm",
              "settings": {
                "theme": "dark"
              }
            }
          }
        ]
      }
    ));

    let ctx = AqlContext::new(g, json.to_object_ref());
    let refs = ctx.refs;

    assert!(
      refs.get("a").unwrap().eq(
        Json::from(json!({
          "$ref": "a",
          "a": "b"
        }))
        .to_object_ref()
      )
    );

    assert!(
      refs.get("b").unwrap().eq(
        Json::from(json!({
          "$ref": "b",
          "username": "Steve",
          "name": {
            "first": "John",
            "last": "Smith"
          }
        }))
        .to_object_ref()
      )
    );

    assert!(
      refs.get("c").unwrap().eq(
        Json::from(json!({
          "$ref": "c",
          "data": {
            "age": 32,
            "height": "50cm",
            "settings": {
              "theme": "dark"
            }
          }
        }))
        .to_object_ref()
      )
    );
  }
}
