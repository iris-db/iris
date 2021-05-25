#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::conn::server::Server;
use crate::io::filesystem::DatabasePath;
use crate::io::logger::s_log;
use crate::io::logger::EventSeverity::Info;
use crate::io::{filesystem, logger};
use std::env;

mod conn;
mod database;
#[allow(warnings, unused)]
mod generated;
mod io;
mod iql;
mod lib;
#[allow(unused_imports)]
mod test_setup;
use rand::Rng;

use crate::io::logger::EventCategory::{ConnPool, Filesystem, General};
use serde_json::json;

fn main() {
	s_log(Info, General, "Starting IrisDB v0.0.1");

	filesystem::prepare();

	for dir in DatabasePath::paths() {
		s_log(
			Info,
			Filesystem,
			&*format!(
				"[Directory-InUse] {}/{}",
				env::current_dir().unwrap().to_str().unwrap(),
				dir.path()
			),
		);
	}

	s_log(
		Info,
		ConnPool,
		&*format!(
			"[Connection-Info] {}",
			json!({
				"instanceId": rand::thread_rng().gen_range(0..u16::MAX),
				"port": 12712,
				"shardCount": 0
			})
			.to_string()
		),
	);

	let s = Server::new(12712, 0);
	s.start();
}
