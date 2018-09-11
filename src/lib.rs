//! Screenshot capturing utility.

#![deny(missing_docs)]

#[macro_use]
extern crate failure;
extern crate imlib2_sys;
extern crate libc;
extern crate png;
#[macro_use]
extern crate structopt;
extern crate time;
extern crate x11;

mod capture;
mod errors;
mod gui;
mod options;
mod window;

pub mod imlib2;
pub mod xlib;

use structopt::StructOpt;
use xlib::Rectangle;
use errors::ScreenshotError;

pub use capture::capture;
pub use options::{Options, Region};
pub use window::SelectWindow;
