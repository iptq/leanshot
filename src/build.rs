fn main() {
    println!("cargo:rustc-link-lib=slop");
    println!("cargo:rustc-link-search=native=/usr/lib");
}
