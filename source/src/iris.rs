#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::io::filesystem;
use crate::server::tcp;

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
	filesystem::prepare();
	// http::server::start();
	tcp::server::start();
}
