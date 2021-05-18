use crate::database::graph::Graph;
use crate::io::logger;
use crate::io::logger::s_log;
use crate::io::logger::EventCategory::Network;
use crate::io::logger::EventSeverity::Info;
use crate::iql::keyword::Keyword;
use rocket::config::{Environment, LoggingLevel};
use rocket::Config;
use std::collections::HashMap;
use std::sync::Mutex;

/// An IrisDB server.
pub struct Server {
	max_cache_size: usize,
	port: u16,
}

impl Server {
	pub fn new(port: u16, max_cache_size: usize) -> Self {
		Self {
			port,
			max_cache_size,
		}
	}

	/// Starts the HTTP server.
	pub fn start(self) {
		let config = Config::build(Environment::Staging)
			.port(self.port)
			.log_level(LoggingLevel::Off)
			.finalize()
			.unwrap();

		s_log(
			Info,
			Network,
			&*format!("HTTP protocol started on http://localhost:{}", self.port),
		);

		rocket::custom(config)
			.mount("/", routes![])
			.manage(Mutex::new(self))
			.launch();
	}
}

/// An in memory representation of the database.
pub struct Database {
	graphs: HashMap<String, Graph>,
}

impl Database {
	/// Calculates the total amount of memory being used by the internal database.
	fn mem_size() -> u32 {
		0
	}

	fn graphs(&mut self) -> &mut HashMap<String, Graph> {
		&mut self.graphs
	}
}
