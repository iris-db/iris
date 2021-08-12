use crate::lib::json::types::JsonObject;
use crate::storage::collection::Collection;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// An action is something that mutates a database collection.
///
/// Example: `POST http://localhost:4000/<collectionName>/<actionName>`.
/// The action data is in the request body as a JSON object.
pub trait CollectionAction<I, O>
where
    I: DeserializeOwned,
    O: Serialize,
{
    /// The action name as a string.
    fn name(&self) -> String;
    /// The logic to perform when the action is dispatched.
    fn handle(&self, ctx: CollectionActionContext<I>) -> O;
}

pub struct QueryFormat {
    query: String,
    opts: JsonObject,
}

/// A wrapper for all context objects when executing a collection action.
pub struct CollectionActionContext<I>
where
    I: DeserializeOwned,
{
    input: I,
    collection: Collection,
}

pub mod actions {
    use super::{CollectionAction, CollectionActionContext};
    use crate::lib::json::types::JsonObject;
    use serde::{Deserialize, Serialize};

    /// Create a new document and store it in a collection.
    pub struct Insert;

    #[derive(Deserialize, Serialize)]
    pub struct InsertInput {
        pub data: JsonObject,
    }

    #[derive(Serialize)]
    pub struct InsertOutput {}

    impl CollectionAction<InsertInput, InsertOutput> for Insert {
        fn name(&self) -> String {
            "Insert".to_string()
        }

        fn handle(&self, ctx: CollectionActionContext<InsertInput>) -> InsertOutput {
            let CollectionActionContext { input, collection } = ctx;

            InsertOutput {}
        }
    }
}
