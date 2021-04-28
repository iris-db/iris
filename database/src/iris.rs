#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::aql::http;
use crate::lib::filesystem;

mod aql;
mod graph;
mod lib;

fn main() {
  filesystem::prepare();
  http::start_rest_server();
}
