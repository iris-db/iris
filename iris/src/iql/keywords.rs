use crate::iql::keyword::{Keyword, KeywordExecutionError};

pub struct Insert;

impl Keyword for Insert {
	fn key(&self) -> String {
		"INSERT".to_string()
	}

	fn exec(&self) -> Result<(), KeywordExecutionError> {
		println!("Executed INSERT action");
		Ok(())
	}
}
