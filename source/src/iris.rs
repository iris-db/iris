#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::io::filesystem;
use crate::server::http;

mod generated;
mod graph;
mod io;
mod lib;
mod query;
mod server;
mod test_lifecycle;

fn main() {
  filesystem::prepare();
  http::server::start();
}
