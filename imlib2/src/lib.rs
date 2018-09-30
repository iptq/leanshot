//! Safe-ish bindings to imlib2 (at least the only parts I need).

#[macro_use]
extern crate failure;
extern crate imlib2_sys;
extern crate leanshot_xlib as xlib;

mod errors;
mod image;

pub use errors::Error;
pub use image::Image;
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

/// Set the visual for the imlib context.
pub fn context_set_image(image: &Image) {
    unsafe { imlib2_sys::imlib_context_set_image(image.as_raw() as imlib2_sys::Imlib_Image) };
}

/// Get a pointer to the raw image data for the current image.
pub fn image_get_data() -> *mut u32 {
    unsafe { imlib2_sys::imlib_image_get_data_for_reading_only() }
}

/// Create cropped image
pub fn create_cropped_image(x: i32, y: i32, width: u32, height: u32) -> Result<Image, Error> {
    let inner =
        unsafe { imlib2_sys::imlib_create_cropped_image(x, y, width as i32, height as i32) };
    if inner.is_null() {
        return Err(Error::Error);
    }
    Ok(Image { inner })
}

/// Create scaled cropped image
pub fn create_scaled_cropped_image(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<Image, Error> {
    let inner = unsafe {
        imlib2_sys::imlib_create_cropped_scaled_image(
            x,
            y,
            width as i32,
            height as i32,
            width as i32,
            height as i32,
        )
    };
    if inner.is_null() {
        return Err(Error::Error);
    }
    Ok(Image { inner })
}
