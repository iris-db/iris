#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::io::filesystem;
use crate::io::filesystem::DatabasePath;
use crate::log::logger;
use crate::log::logger::LogSeverity::Info;
use crate::server::tcp;
use std::env;

#[allow(warnings, unused)]
mod generated;
mod graph;
mod io;
mod iql;
mod lib;
mod log;
mod server;
#[allow(unused_imports)]
mod test_lifecycle;

fn main() {
	logger::b_log(Info, "Starting IrisDB Community Edition");

	logger::b_log(Info, "Preparing filesystem");
	filesystem::prepare();

	for dir in DatabasePath::paths() {
		logger::b_log(
			Info,
			&*format!(
				"Using directory {}/{}",
				env::current_dir().unwrap().to_str().unwrap(),
				dir.path()
			),
		);
	}

	logger::b_log(Info, "Connecting to cluster nodes...");
	logger::b_log(Info, "Cluster statistics: 1 connected node");

	logger::b_log(Info, "Starting TCP server");
	tcp::server::start();
}
