use std::cmp::Ordering;

use crate::aql::context::AqlContext;
use crate::lib::bson::JsonObject;

pub type DirectiveList = Vec<Box<dyn Directive>>;

pub const DIRECTIVE_PREFIX: &str = "$";

/// A prefixed JSON key that executes a database query.
pub trait Directive: Sync + Send {
  /// The key name. Not the actual formatted JSON key.
  fn key(&self) -> &str;
  /// Execute the directive's action.
  fn exec(&self, ctx: &AqlContext) -> JsonObject;
}

impl Ord for dyn Directive {
  fn cmp(&self, other: &Self) -> Ordering {
    self.key().cmp(other.key())
  }
}

impl PartialOrd for dyn Directive {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.key().partial_cmp(other.key())
  }
}

impl Eq for dyn Directive {}

impl PartialEq for dyn Directive {
  fn eq(&self, other: &Self) -> bool {
    self.key().eq(other.key())
  }
}
