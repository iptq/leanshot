#[macro_use]
extern crate failure;
extern crate imlib2;
extern crate png;
#[macro_use]
extern crate structopt;
extern crate time;
extern crate xlib;

mod capture;
mod errors;
mod gui;
mod options;

use structopt::StructOpt;

pub use capture::capture;
pub use options::Options;

#[derive(Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}
