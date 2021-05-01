extern crate bindgen;
use std::path::PathBuf;

const EXTERN_LIB_PATH: &str = "build";

fn main() {
  link_library("expressions");
}

fn link_library(lib: &str) {
  let bindings = bindgen::Builder::default()
    .header(format!("{}/{}/lib{}.h", EXTERN_LIB_PATH, lib, lib))
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from("src/generated");
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  println!(
    "cargo:rustc-link-search=native={}",
    format!("{}/{}", EXTERN_LIB_PATH, lib)
  );
  println!("cargo:rustc-link-lib=static={}", lib);
}
