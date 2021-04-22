use std::collections::HashMap;

use serde_json::Value;

use crate::graph::node_plane::NodePlane;
use crate::lib::bson::JsonObject;

/// The JSON key that denotes a reference.
const REF_KEY: &str = "$ref";

/// Holds the refs and other metadata about the request.
pub struct AqlContext<'a> {
    /// Current node plane.
    plane: &'a NodePlane,
    /// JSON object references in the request body.
    refs: HashMap<String, JsonObject>,
}

impl AqlContext<'_> {
    pub fn new(plane: &NodePlane, body: JsonObject) -> AqlContext {
        AqlContext {
            plane,
            refs: AqlContext::traverse_refs(body),
        }
    }

    pub fn plane(&self) -> &NodePlane {
        &self.plane
    }

    pub fn refs(&self) -> &HashMap<String, JsonObject> {
        &self.refs
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
        let p = NodePlane::new("TEST");

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

        let ctx = AqlContext::new(&p, json.as_object().unwrap().clone());
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
