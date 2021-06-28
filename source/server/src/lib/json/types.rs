use serde::de::IntoDeserializer;
use serde::Deserialize;
use serde_json::{Map, Value};

pub type JsonObject = Map<String, Value>;

/// Utility struct for handling common JSON operations, extending the functionality of the
/// serde_json library.
pub struct SmartJson {
    /// Internal json value.
    inner: Value,
}

impl SmartJson {
    /// Converts to a JsonObject in an unchecked manner.
    ///
    /// Only should be called if it is positive that the json value is an object.
    pub fn to_object_unchecked(&self) -> &JsonObject {
        self.inner.as_object().unwrap()
    }

    /// Internal json value.
    pub fn inner(&self) -> &Value {
        &self.inner
    }

    /// Deserializes a json value into a typed struct.
    pub fn into_struct<'a, T>(self) -> Result<T, JsonDeserializationError>
    where
        T: Deserialize<'a>,
    {
        let des = self.inner.into_deserializer();
        let res: Result<T, _> = serde_path_to_error::deserialize(des);

        return match res {
            Ok(value) => Ok(value),
            Err(e) => Err(JsonDeserializationError {
                path: e.path().to_string(),
                expected_type: "".to_string(),
                msg: e.to_string(),
                inner: e,
            }),
        };
    }
}

impl From<Value> for SmartJson {
    fn from(v: Value) -> Self {
        SmartJson { inner: v }
    }
}

type SerdeDeserializationError = serde_path_to_error::Error<serde_json::Error>;

pub struct JsonDeserializationError {
    pub path: String,
    pub expected_type: String,
    pub msg: String,
    inner: SerdeDeserializationError,
}

impl JsonDeserializationError {
    /// Retrieve the raw deserialization error from serde.
    pub fn into_inner(self) -> SerdeDeserializationError {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_into_struct_ok() {
        #[derive(Deserialize)]
        struct Name {
            first_name: String,
            last_name: String,
        }

        let json: SmartJson = json!({
            "first_name": "John",
            "last_name": "Smith"
        })
        .into();

        let name: Name = json.into_struct().ok().unwrap();
        assert_eq!(name.first_name, "John");
        assert_eq!(name.last_name, "Smith");
    }

    #[test]
    fn test_into_struct_err() {
        #[derive(Deserialize)]
        struct Name {
            first_name: String,
            last_name: String,
        }

        let json: SmartJson = json!({
            "first_name": {},
            "last_name": "Smith"
        })
        .into();

        let err = json.into_struct::<Name>().err().expect("type error");

        println!("{}", err.msg);
        // assert_eq!(err.path, "first_name");
    }
}
