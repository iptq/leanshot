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
mod window;

use structopt::StructOpt;
use xlib::Rectangle;

pub use capture::capture;
pub use options::Options;
pub use window::SelectWindow;
