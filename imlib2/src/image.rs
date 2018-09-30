use std::ffi::CString;
use std::path::Path;

use imlib2_sys as im;

use xlib::Drawable;
use Error;

/// A simple wrapper around Imlib_Image
pub struct Image {
    pub(crate) inner: im::Imlib_Image,
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

    /// Get width
    pub fn get_width(&self) -> i32 {
        unsafe {
            im::imlib_context_set_image(self.inner);
            im::imlib_image_get_width()
        }
    }

    /// Get height
    pub fn get_height(&self) -> i32 {
        unsafe {
            im::imlib_context_set_image(self.inner);
            im::imlib_image_get_height()
        }
    }

    /// Save this image
    pub fn save_image(&self, file: impl AsRef<Path>) -> Result<(), Error> {
        let mut error = 0;
        let path = CString::new(file.as_ref().to_str().unwrap()).unwrap();
        unsafe { im::imlib_save_image_with_error_return(path.as_ptr(), &mut error) };
        Ok(())
    }

    /// Get raw image
    pub fn as_raw(&self) -> im::Imlib_Image {
        self.inner
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            im::imlib_context_set_image(self.inner);
            im::imlib_free_image();
        }
    }
}
