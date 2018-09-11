//! Safe-ish bindings to parts of x11's xlib module.
//!
//! I need this for my project.

#![deny(missing_docs)]

#[macro_use]
extern crate failure;
extern crate libc;
extern crate x11;

mod display;
mod error;
mod image;
mod window;

pub use display::Display;
pub use error::X11Error;
pub use image::{Image, ImageByteOrder, PixBuffer};
pub use window::{Window, WindowAttributes};
