extern crate bindgen;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const EXTERN_LIB_PATH: &str = "build";

fn main() {
  link_library("expressions");
}

fn link_library(lib: &str) {
  let bindings_file_name = format!("{}_bindings", lib);
  let bindings_file = format!("{}{}", bindings_file_name, ".rs");

  let module_name = format!("pub mod {};", bindings_file_name);

  let bindings = bindgen::Builder::default()
    .header(format!("{}/{}/lib{}.h", EXTERN_LIB_PATH, lib, lib))
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from("src/generated");
  bindings
    .write_to_file(out_path.join(bindings_file))
    .expect("Could not generate bindings");

  let mut mod_file = OpenOptions::new()
    .truncate(true)
    .write(true)
    .create(true)
    .open("src/generated/mod.rs")
    .expect("Could not open mod file for generated");

  mod_file
    .write(module_name.as_bytes())
    .expect(format!("Could not write lib {} to generated module file", lib).as_str());

  println!(
    "cargo:rustc-link-search=native={}",
    format!("{}/{}", EXTERN_LIB_PATH, lib)
  );
  println!("cargo:rustc-link-lib=static={}", lib);
}
