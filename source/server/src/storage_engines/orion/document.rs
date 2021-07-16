use crate::lib::json::types::JsonObject;

/// A JSON document, stored internally using BSON encoding.
pub struct Document {
    inner: JsonObject,
}

impl Document {
    pub fn new(o: JsonObject) -> Document {
        Document { inner: o }
    }

    pub fn as_json(&self) -> &JsonObject {
        &self.inner
    }
}

impl From<JsonObject> for Document {
    fn from(o: JsonObject) -> Self {
        Document::new(o)
    }
}
