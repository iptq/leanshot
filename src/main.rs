extern crate screenshot;

use std::process::exit;

fn main() {
    match screenshot::run() {
        Ok(()) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        }
    }
}
