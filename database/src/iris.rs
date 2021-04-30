#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::server::http;
use io::filesystem;

mod graph;
mod io;
mod lib;
mod server;

fn main() {
  filesystem::prepare();
  http::server::start();
}
