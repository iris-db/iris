use serde::Serialize;

use crate::api::collection_action::CollectionAction;
use crate::page::page_set::PageSet;
use crate::storage::utils::CollectionNameFormatter;
use serde::de::DeserializeOwned;

/// An abstraction over data pages.
pub struct Collection {
    name: CollectionNameFormatter,
    pages: PageSet,
}

impl Collection {
    pub fn new(name: CollectionNameFormatter) -> Self {
        todo!()
    }

    pub fn name(&self) -> &CollectionNameFormatter {
        &self.name
    }

    /// Dispatches a collection action, returning the output.
    pub fn dispatch_action<I, O, A>(self, action: A)
    where
        I: DeserializeOwned,
        O: Serialize,
        A: CollectionAction<I, O>,
    {
        todo!()
        // action.handle();
    }
}
