extern crate cmake;

fn main() {
    let dst = cmake::build("libscreenshot");
    println!("cargo:rustc-link-lib=static=screenshot");
    println!("cargo:rustc-link-search=native={}/libs", dst.display());
}
