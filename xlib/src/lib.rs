//! Safe-ish bindings to parts of x11's xlib module.
//!
//! I need this for my project.

#![deny(missing_docs)]

#[macro_use]
extern crate failure;
extern crate imlib2;
extern crate libc;
extern crate x11;

mod display;
mod errors;
mod image;
mod visual;
mod rect;
mod window;

pub use display::Display;
pub use errors::X11Error;
pub use image::{Image, ImageByteOrder, PixBuffer};
pub use visual::Visual;
pub use rect::Rectangle;
pub use window::{Window, WindowAttributes};

