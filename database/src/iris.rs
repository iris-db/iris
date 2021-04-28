#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::query::http;
use io::filesystem;

mod graph;
mod io;
mod lib;
mod query;

fn main() {
  filesystem::prepare();
  http::server::start();
}
