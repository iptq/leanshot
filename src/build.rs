fn main() {
    println!("cargo:rustc-link-lib=slopy");
    println!("cargo:rustc-link-search=native=/usr/lib");
}
