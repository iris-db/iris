#![feature(map_first_last)]
#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::server::http;
use io::filesystem;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod graph;
mod io;
mod lib;
mod query;
mod server;

extern "C" {
  fn HelloWorld() -> *const c_char;
}

#[repr(C)]
struct GoString {
  a: *const c_char,
  b: i64,
}

fn main() {
  let res = unsafe { CStr::from_ptr(HelloWorld()) };
  let x = res.to_str().expect("ABC");
  println!("{}", x);

  // filesystem::prepare();
  // http::server::start();
}
