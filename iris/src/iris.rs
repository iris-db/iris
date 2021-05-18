#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::io::filesystem;
use crate::io::filesystem::DatabasePath;

mod conn;
mod database;
#[allow(warnings, unused)]
mod generated;
mod io;
mod iql;
mod lib;
#[allow(unused_imports)]
mod test_setup;

fn main() {
	println!("Hello, world!");
}
