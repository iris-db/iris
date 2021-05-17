use crate::graph::graph::Graph;
use crate::io::page;
use crate::lib::bson::JsonObject;
use serde::{Deserialize, Serialize};

/// An Iris Query Language command, such as insert or delete.
trait Command {
	/// They JSON key of the command.
	fn key() -> String;
	/// Executes the command.
	fn exec(graph: &mut Graph, data: &JsonObject) -> Result<JsonObject, CommandError>;
}

pub enum CommandError {
	/// Improper JSON type received.
	ImproperType(String),
	/// Missing a required key.
	MissingRequiredAttribute(String),
	/// Could not write to the data page.
	PageWriteError(page::WriteError),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InsertCommand {}

impl Command for InsertCommand {
	fn key() -> String {
		String::from("insert")
	}

	fn exec(graph: &mut Graph, data: &JsonObject) -> Result<JsonObject, CommandError> {
		Err(CommandError::ImproperType("Array".to_string()))
	}
}
