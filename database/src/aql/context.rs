use std::collections::HashMap;

use serde_json::Value;

use crate::aql::directive::{
  Directive, DirectiveDataExtraction, DirectiveDataSet, DirectiveErrorType,
};
use crate::graph::graph::Graph;
use crate::lib::bson::{values_to_objects, JsonObject};

/// The JSON key that denotes a reference to an object.
const REF_KEY: &str = "$ref";

/// Holds the refs and other metadata about the request.
pub struct HttpContext<'a> {
  pub graph: &'a mut Graph,
  pub data: DirectiveDataSet,
  pub refs: HashMap<String, JsonObject>,
}

impl HttpContext<'_> {
  pub fn try_new<'a>(
    graph: &'a mut Box<Graph>,
    directive: &'static dyn Directive,
    body: &JsonObject,
  ) -> Result<HttpContext<'a>, DirectiveErrorType> {
    let data = HttpContext::retrieve_data(directive, body)?;

    Ok(HttpContext {
      graph,
      data,
      refs: HttpContext::traverse_refs(body),
    })
  }

  fn retrieve_data(
    directive: &'static dyn Directive,
    body: &JsonObject,
  ) -> Result<DirectiveDataSet, DirectiveErrorType> {
    let data = HttpContext::extract_directive_data(directive, body);
    let data = match data {
      DirectiveDataExtraction::Array(v) => v,
      _ => return Err(DirectiveErrorType::ExpectedArray),
    };

    Ok(DirectiveDataSet::new(data))
  }

  /// Extracts directive data by looking up the directive JSON key's value on the request body.
  fn extract_directive_data<'a>(
    directive: &'static dyn Directive,
    data: &'a JsonObject,
  ) -> DirectiveDataExtraction<'a> {
    let key = directive.key();

    let data = data.get(key).unwrap();

    return match data {
      Value::Array(v) => DirectiveDataExtraction::Array(values_to_objects(v)),
      Value::Object(v) => DirectiveDataExtraction::Object(v.clone()),
      v => DirectiveDataExtraction::Other(v),
    };
  }

  /// Traverses each JSON object for a ref key.
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

  use crate::aql::directive::{Directive, DirectiveResult};
  use crate::lib::bson::Json;

  use super::*;

  #[test]
  fn test_extract_directive_data() {
    let g = &mut Graph::new("TEST");

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

    struct TestDirective;

    impl Directive for TestDirective {
      fn key(&self) -> &str {
        "insert"
      }

      fn exec(&self, ctx: HttpContext) -> DirectiveResult {
        todo!()
      }
    }

    let ctx = HttpContext::try_new(g, &TestDirective, json.to_object_ref())
      .ok()
      .expect("Could not create HttpContext");

    let data = ctx.data;

    data
      .dispatch::<()>(|o| {
        assert!(
          o.data().eq(
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

        Ok(())
      })
      .ok()
      .expect("Could not dispatch actions");
  }

  #[test]
  fn test_ref_traversal() {
    let g = &mut Graph::new("TEST");

    let json = Json::from(json!(
      {
        "get": [
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

    struct TestDirective;

    impl Directive for TestDirective {
      fn key(&self) -> &str {
        "insert"
      }

      fn exec(&self, _ctx: HttpContext) -> DirectiveResult {
        todo!()
      }
    }

    let ctx = HttpContext::try_new(g, &TestDirective, json.to_object_ref())
      .ok()
      .expect("Could not create HttpContext");
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
