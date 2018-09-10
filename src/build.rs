fn main() {
    println!("cargo:rustc-link-lib=slopy");
    println!("cargo:rustc-link-search=native=/usr/lib:/usr/lib/x86_64-linux-gnu");
}
