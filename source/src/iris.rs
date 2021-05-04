#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::io::filesystem;
use crate::server::http;
use crate::server::tcp;

mod generated;
mod graph;
mod io;
mod iql;
mod lib;
mod server;
mod test_lifecycle;
mod log;

fn main() {
	filesystem::prepare();
	// http::server::start();
	tcp::server::start();
}
