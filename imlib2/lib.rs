//! Safe-ish bindings to imlib2 (at least the only parts I need).

#![deny(missing_docs)]

#[macro_use]
extern crate failure;
extern crate imlib2_sys;
extern crate x11;

mod errors;
mod image;

pub use errors::Error;
pub use image::Image;
pub use imlib2_sys::{Drawable, Pixmap};
