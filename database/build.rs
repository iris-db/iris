const EXTERN_LIB_PATH: &str = "build";

fn main() {
  link_library("expressions");
}

fn link_library(lib: &str) {
  println!(
    "cargo:rustc-link-search=native={}",
    [EXTERN_LIB_PATH, "/", lib].concat()
  );
  println!("cargo:rustc-link-lib=static={}", lib);
}
