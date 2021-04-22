#![feature(option_insert)]
#![feature(map_first_last)]
#![feature(box_syntax)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::aql::http;
use crate::lib::filesystem;

mod aql;
mod graph;
mod io;
mod lib;

#[tokio::main]
async fn main() {
    filesystem::prepare();
    http::start_rest_server();
}
