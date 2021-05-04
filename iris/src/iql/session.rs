use crate::iql::lex::IqlValue;
use std::collections::HashMap;

/// Stores connection session data.
pub struct Session {
	/// Currently defined variables. Only applies to shell connections.
	vars: HashMap<String, IqlValue>,
	/// Session connection method.
	conn_method: ConnectionMethod,
}

/// Describes what kind of connection is being maintained to the database.
pub enum ConnectionMethod {
	/// A HTTP connection usually through the http server.
	Http,
	/// A TCP connection usually through the IQL shell or a language SDK.
	Tcp(TcpClientType),
}

/// Describes a client that is connected to the database.
pub enum TcpClientType {
	/// The IQL shell.
	Shell,
	/// A language driver / SDK.
	LanguageDriver,
}
