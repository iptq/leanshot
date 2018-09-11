#[macro_use]
extern crate failure;
extern crate png;
#[macro_use]
extern crate structopt;
extern crate time;
extern crate xlib;

mod capture;
mod errors;
mod gui;
mod image;
mod options;

use structopt::StructOpt;

pub use capture::capture;
pub use image::ImageExt;
pub use options::Options;

#[derive(Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
