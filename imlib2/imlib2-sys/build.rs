// bindgen build script by remexre

extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::Builder;

fn main() {
    println!("cargo:rustc-link-lib=Imlib2");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    Builder::default()
        .header("wrapper.h")
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
    .expect("Unable to write bindings");
}
