#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(trait_alias)]

#[macro_use]
extern crate rocket;

use crate::http::{config::HttpServerConfig, server::HttpServer};
use crate::io::logger::s_log;
use crate::io::logger::EventSeverity::Info;
use crate::io::path;
use crate::io::path::DatabasePath;
use std::env;

mod api;
mod http;
#[allow(warnings, unused)]
mod io;
mod lib;
mod page;
mod storage;
use rand::Rng;

use crate::io::logger::EventCategory::{ConnPool, Filesystem, General};
use serde_json::json;

fn main() {
    s_log(Info, General, "Starting IrisDB v0.0.1");

    path::prepare();

    for dir in DatabasePath::paths() {
        s_log(
            Info,
            Filesystem,
            &*format!(
                "[Directory-InUse] {}/{}",
                env::current_dir().unwrap().to_str().unwrap(),
                dir.path_name()
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

    let s = HttpServer::new(HttpServerConfig { port: 12712 });
    s.start();
}
