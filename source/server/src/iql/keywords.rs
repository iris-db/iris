use crate::iql::keyword::{DispatchKeywordContext, Keyword, KeywordError};
use crate::lib::json::types::JsonObject;

pub struct Insert;

impl Keyword for Insert {
	fn key(&self) -> String {
		"insert".into()
	}

	fn exec(&self, ctx: DispatchKeywordContext) -> Result<JsonObject, KeywordError> {
		unimplemented!()
	}
}
