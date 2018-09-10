#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;
extern crate time;
extern crate x11;

mod capture;
mod errors;
mod gui;
mod image;
mod options;

use failure::Error;
use structopt::StructOpt;

use options::Options;

pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn run() -> Result<(), Error> {
    let opt = Options::from_args();
    capture::capture(&opt)?;
    Ok(())
}
