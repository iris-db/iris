use std::collections::HashMap;

use crate::aql::directive::{Directive, DIRECTIVE_PREFIX};
use crate::graph::graph::Graph;
use crate::lib::bson::{map_values, JsonObject};
use serde_json::Value;

/// The JSON key that denotes a reference.
const REF_KEY: &str = "$ref";

/// Holds the refs and other metadata about the request.
pub struct AqlContext<'a> {
    /// Current node graph.
    graph: &'a Graph,
    /// JSON object that contains the directive data. It is an array.
    data: &'a JsonObject,
    /// JSON object references in the request body.
    refs: HashMap<String, JsonObject>,
}

/// The result of data extraction from the POST body for the directive.
pub enum DirectiveDataExtraction<'a> {
    /// JSON object.
    Object(&'a JsonObject),
    /// JSON array.
    Array(Vec<&'a JsonObject>),
    /// Other JSON type.
    Other,
}

impl AqlContext<'_> {
    pub fn new<'a>(graph: &'a Graph, body: &'a JsonObject) -> AqlContext<'a> {
        AqlContext {
            graph,
            data: body,
            refs: AqlContext::traverse_refs(body.clone()),
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn refs(&self) -> &HashMap<String, JsonObject> {
        &self.refs
    }

    /// Extracts directive data by looking up the directive JSON key's value on the request body.
    pub fn extract_directive_data(&self, directive: &dyn Directive) -> DirectiveDataExtraction {
        let key = format!("{}{}", DIRECTIVE_PREFIX, directive.key());

        let data = self.data.get(key.as_str()).unwrap();

        return match data {
            Value::Array(v) => DirectiveDataExtraction::Array(map_values(v)),
            Value::Object(v) => DirectiveDataExtraction::Object(v),
            _ => DirectiveDataExtraction::Other,
        };
    }

    /// Traverses each JSON object for a reference.
    fn traverse_refs(json: JsonObject) -> HashMap<String, JsonObject> {
        fn traverse_object(tree: &mut HashMap<String, JsonObject>, ch: JsonObject) {
            for (k, v) in &ch {
                if k == REF_KEY && v.is_string() {
                    tree.insert(v.as_str().unwrap().to_string(), ch.clone());
                }

                if v.is_object() {
                    traverse_object(tree, v.as_object().unwrap().clone());
                    return;
                }

                if v.is_array() {
                    for v in v.as_array().unwrap() {
                        traverse_object(tree, v.as_object().unwrap().clone());
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

    use super::*;

    #[test]
    fn test_ref_traversal() {
        let p = Graph::new("TEST");

        let json = json!(
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
        );

        let ctx = AqlContext::new(&p, &json.as_object().unwrap());
        let refs = ctx.refs();

        assert!(refs.get("a").unwrap().eq(json!({
            "$ref": "a",
            "a": "b"
        })
        .as_object()
        .unwrap()));

        assert!(refs.get("b").unwrap().eq(json!({
            "$ref": "b",
            "username": "Steve",
            "name": {
                "first": "John",
                "last": "Smith"
            }
        })
        .as_object()
        .unwrap()));

        assert!(refs.get("c").unwrap().eq(json!({
            "$ref": "c",
            "data": {
                "age": 32,
                "height": "50cm",
                "settings": {
                    "theme": "dark"
                }
            }
        })
        .as_object()
        .unwrap()));
    }
}
