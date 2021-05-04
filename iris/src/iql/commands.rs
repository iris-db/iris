use crate::graph::node::Node;
use crate::iql::lex::{Command, CommandContext, CommandResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Insert;

#[derive(Serialize, Deserialize)]
pub struct InsertFlags {
	pub group: Option<String>,
	pub data: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertResult {
	pub time: u64,
}

impl Command<InsertFlags, InsertResult> for Insert {
	fn name() -> &'static str {
		"Insert"
	}

	fn exec(ctx: CommandContext, flags: InsertFlags) -> CommandResult<InsertResult> {
		let CommandContext { graph } = ctx;
		let InsertFlags { group, data } = flags;

		let id = graph.next_id();

		let time = graph.insert(Node::new(id, group, data, None))?;

		Ok(vec![InsertResult { time: time as u64 }])
	}
}
