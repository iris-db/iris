use std::collections::HashMap;

use serde_json::Value;

use crate::graph::graph::Graph;
use crate::lib::bson::{values_to_objects, JsonObject};
use crate::query::directive::{Directive, DirectiveDataSet, DirectiveError};

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
    graph: &'a mut Graph,
    directive: &'static dyn Directive,
    body: &JsonObject,
  ) -> Result<HttpContext<'a>, DirectiveError> {
    let data = HttpContext::lookup_directive_data(directive, body)?;

    Ok(HttpContext {
      graph,
      data,
      refs: HttpContext::traverse_refs(body),
    })
  }

  /// Extracts directive data by looking up the directive JSON key's value on a JSON object.
  fn lookup_directive_data(
    directive: &'static dyn Directive,
    body: &JsonObject,
  ) -> Result<DirectiveDataSet, DirectiveError> {
    let data = body.get(directive.key()).unwrap();

    let data = match data {
      Value::Array(v) => v,
      _ => return Err(DirectiveError::ExpectedArray),
    };

    Ok(DirectiveDataSet::new(values_to_objects(data)))
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

  use crate::lib::bson::JsonObjectWrapper;
  use crate::query::directive::{Directive, DirectiveResult};
  use crate::use_test_filesystem;

  use super::*;

  #[test]
  fn test_extract_directive_data() {
    use_test_filesystem!();

    let g = &mut Graph::new("TEST").expect("Error while creating graph");

    let json = JsonObjectWrapper::from(json!(
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

      fn exec(&self, _ctx: HttpContext) -> DirectiveResult {
        todo!()
      }
    }

    let ctx = HttpContext::try_new(g, &TestDirective, json.convert_ref())
      .ok()
      .expect("Could not create HttpContext");

    let data = ctx.data;

    data
      .dispatch::<(), _>(|o| {
        assert!(
          o.data().eq(
            JsonObjectWrapper::from(json!({
              "$ref": "c",
              "data": {
                "age": 32,
                "height": "50cm",
                "settings": {
                  "theme": "dark"
                }
              }
            }))
            .convert_ref()
          )
        );

        Ok(())
      })
      .ok()
      .expect("Could not dispatch actions");
  }

  #[test]
  fn test_ref_traversal() {
    use_test_filesystem!();

    let g = &mut Graph::new("TEST").expect("Error while creating graph");

    let json = JsonObjectWrapper::from(json!(
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

    let ctx = HttpContext::try_new(g, &TestDirective, json.convert_ref())
      .ok()
      .expect("Could not create HttpContext");
    let refs = ctx.refs;

    assert!(
      refs.get("a").unwrap().eq(
        JsonObjectWrapper::from(json!({
          "$ref": "a",
          "a": "b"
        }))
        .convert_ref()
      )
    );

    assert!(
      refs.get("b").unwrap().eq(
        JsonObjectWrapper::from(json!({
          "$ref": "b",
          "username": "Steve",
          "name": {
            "first": "John",
            "last": "Smith"
          }
        }))
        .convert_ref()
      )
    );

    assert!(
      refs.get("c").unwrap().eq(
        JsonObjectWrapper::from(json!({
          "$ref": "c",
          "data": {
            "age": 32,
            "height": "50cm",
            "settings": {
              "theme": "dark"
            }
          }
        }))
        .convert_ref()
      )
    );
  }
}
