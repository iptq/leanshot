use std::ffi::CString;
use std::path::Path;

use imlib2_sys as im;

use imlib2::Error;
use xlib::Drawable;

/// A simple wrapper around Imlib_Image
pub struct Image {
    inner: im::Imlib_Image,
}

impl Image {
    /// Creates an Image from a pixmap.
    pub fn create_from_drawable(
        drawable: impl Drawable,
        pixmap: im::Pixmap,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        grab_x: bool,
    ) -> Result<Self, Error> {
        unsafe { im::imlib_context_set_drawable(drawable.as_drawable()) };
        let image = unsafe {
            im::imlib_create_image_from_drawable(
                pixmap,
                x,
                y,
                width,
                height,
                if grab_x { 1 } else { 0 },
            ) as im::Imlib_Image
        };
        unsafe { im::imlib_context_set_image(image) };
        Ok(Image { inner: image })
    }

    /// Save this image
    pub fn save_image(&self, file: impl AsRef<Path>) -> Result<(), Error> {
        let mut error = 0;
        let path = CString::new(file.as_ref().to_str().unwrap()).unwrap();
        unsafe { im::imlib_save_image_with_error_return(path.as_ptr(), &mut error) };
        Ok(())
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { im::imlib_free_image() };
    }
}
