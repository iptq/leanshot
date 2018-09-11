//! Safe-ish bindings to imlib2 (at least the only parts I need).

#![deny(missing_docs)]

mod errors;
mod image;

use imlib2_sys;

pub use self::errors::Error;
pub use self::image::Image;
pub use imlib2_sys::{Drawable, Pixmap};

use xlib::{Display, Visual};

/// Set the display for the imlib context.
pub fn context_set_display(display: &Display) {
    unsafe {
        imlib2_sys::imlib_context_set_display(display.as_raw() as *mut imlib2_sys::_XDisplay)
    };
}

/// Set the visual for the imlib context.
pub fn context_set_visual(visual: &Visual) {
    unsafe { imlib2_sys::imlib_context_set_visual(visual.as_raw() as *mut imlib2_sys::Visual) };
}
