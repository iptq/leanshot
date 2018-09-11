use std::ffi::CString;
use std::path::Path;

use imlib2_sys as im;
use x11;

use Error;

/// A simple wrapper around Imlib_Image
pub struct Image {
    inner: im::Imlib_Image,
}

impl Image {
    /// Creates an Image from a pixmap.
    pub fn create_from_drawable(
        drawable: impl AsRef<im::Drawable>,
        pixmap: im::Pixmap,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        grab_x: bool,
    ) -> Result<Self, Error> {
        unsafe { im::imlib_context_set_drawable(*drawable.as_ref()) };
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
        Ok(Image { inner: image })
    }

    /// Save this image
    pub fn save_image(&self, file: impl AsRef<Path>) -> Result<(), Error> {
        unsafe { im::imlib_context_set_image((*self).inner) };
        let mut error = 0;
        let path = CString::new(file.as_ref().to_str().unwrap()).unwrap();
        unsafe { im::imlib_save_image_with_error_return(path.as_ptr(), &mut error) };
        Ok(())
    }
}
