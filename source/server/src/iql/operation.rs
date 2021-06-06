use crate::iql::query::ExecutionContext;

trait Operation {
	fn json_name(&self) -> String;
	fn exec(&self, ctx: ExecutionContext);
}
