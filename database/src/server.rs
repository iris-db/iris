#![feature(option_insert)]
#![feature(map_first_last)]
#![feature(box_syntax)]
#![feature(test)]

use crate::conn::http;
use crate::lib::bson::encode_bson;
use crate::lib::filesystem;
use std::fs;

mod conn;
mod graph;
mod io;
mod lib;

#[tokio::main]
async fn main() {
    filesystem::prepare();
    http::start_http_server().await;
}
