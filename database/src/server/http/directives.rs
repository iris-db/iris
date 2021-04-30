use crate::graph::node::CreateNodeData;
use crate::server::http::context::HttpContext;
use crate::server::http::directive::{Directive, DirectiveError, DirectiveResult};

/// Insert a node into a graph.
pub struct InsertDirective;
