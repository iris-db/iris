use crate::iql::keyword::{DispatchKeywordContext, Keyword, QueryError};
use serde::{Deserialize, Serialize};

pub struct Insert;

#[derive(Serialize, Deserialize)]
pub struct InsertArgs {
	hi: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertResult {
	hi: String,
}

impl Keyword for Insert {
	type Args = InsertArgs;
	type Ok = InsertResult;

	fn key(&self) -> String {
		"insert".to_string()
	}

	fn exec(&self, ctx: DispatchKeywordContext<Self::Args>) -> Result<Self::Ok, QueryError> {
		unimplemented!()
	}
}

pub struct Delete;

#[derive(Serialize, Deserialize)]
pub struct DeleteArgs {
	hi: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteResult {
	hi: String,
}

impl Keyword for Delete {
	type Args = DeleteArgs;
	type Ok = DeleteResult;

	fn key(&self) -> String {
		"delete".to_string()
	}

	fn exec(&self, ctx: DispatchKeywordContext<Self::Args>) -> Result<Self::Ok, QueryError> {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_insert() {
		#[derive(Serialize, Deserialize)]
		pub struct InsertArgs {
			key: String,
		}

		let x: InsertArgs = serde_json::from_str(
			r#"
			{
				"key": "hi",
				"other": "hi2"
			}
		"#,
		)
		.unwrap();
	}
}
