use crate::lib::json::types::JsonObject;

/// Converts a JsonObject into a strongly typed JSON struct.
pub fn from_single<T>(item: JsonObject) -> T
where
    T: From<JsonObject>,
{
    T::from(item)
}

/// Converts a vec of JsonObjects into a vec of strongly typed JSON structs.
pub fn from_vec<T>(items: Vec<JsonObject>) -> Vec<T>
where
    T: From<JsonObject>,
{
    items.into_iter().map(|o| T::from(o)).collect()
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::{json, Value};

    use super::*;

    #[derive(Deserialize, PartialEq, Debug)]
    struct JsonStruct {
        a: u8,
        b: u8,
    }

    impl From<JsonObject> for JsonStruct {
        fn from(o: JsonObject) -> Self {
            let value: Result<Self, _> = serde_json::from_value(Value::from(o));
            value.unwrap_or(JsonStruct { a: 0, b: 0 })
        }
    }

    #[test]
    fn test_from_single() {
        let res: JsonStruct = from_single(
            json!({
                "a": 32,
                "b": 16
            })
            .as_object()
            .unwrap()
            .clone(),
        );

        assert_eq!(res.a, 32);
        assert_eq!(res.b, 16);
    }

    #[test]
    fn test_from_vec() {
        let items: Vec<JsonObject> = vec![
            json!({
                "a": 32,
                "b": 16
            }),
            json!({
                "a": 16,
                "b": 8
            }),
            json!({
                "a": 8,
                "b": 4
            }),
        ]
        .into_iter()
        .map(|v| v.as_object().unwrap().clone())
        .collect();

        let result = from_vec::<JsonStruct>(items);
        assert_eq!(result[0], JsonStruct { a: 32, b: 16 });
        assert_eq!(result[1], JsonStruct { a: 16, b: 8 });
        assert_eq!(result[2], JsonStruct { a: 8, b: 4 });
    }
}
