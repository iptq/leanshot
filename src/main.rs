//! Screenshot capturing utility.

#![deny(missing_docs)]

#[macro_use]
extern crate failure;
extern crate gl;
extern crate glutin;
extern crate imlib2;
extern crate nanovg;
extern crate png;
#[macro_use]
extern crate structopt;
extern crate time;
extern crate leanshot_xlib as xlib;

mod capture;
mod errors;
mod gui;
mod options;

use errors::ScreenshotError;
use structopt::StructOpt;
use xlib::Rectangle;

pub use capture::capture;
pub use options::{Options, Region};

use failure::Error;

#[allow(missing_docs)]
pub fn main() -> Result<(), Error> {
    let opt = Options::from_args();
    capture(&opt).map(|_| ()).map_err(|err| err.into())
}
