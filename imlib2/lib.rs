//! Safe-ish bindings to imlib2 (at least the only parts I need).

#![deny(missing_docs)]

#[macro_use]
extern crate failure;
extern crate imlib2_sys;
extern crate x11;

mod errors;
mod image;
mod visual;

pub use errors::Error;
pub use image::Image;
pub use imlib2_sys::{Drawable, Pixmap};
pub use visual::Visual;

/// Set the display for the imlib context.
pub fn context_set_display(display: *mut x11::xlib::Display) {
    unsafe { imlib2_sys::imlib_context_set_display(display as *mut imlib2_sys::_XDisplay) };
}

/// Set the visual for the imlib context.
pub fn context_set_visual(visual: Visual) {
    unsafe { imlib2_sys::imlib_context_set_visual(visual.inner) };
}

