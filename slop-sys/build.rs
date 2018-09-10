extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("slop").define("CMAKE_SKIP_RPATH", "ON").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=slopy");

    // println!("cargo:rustc-link-lib=slopy");
    // println!("cargo:rustc-link-search=native=/usr/lib/usr/lib/x86_64-linux-gnu");
}
