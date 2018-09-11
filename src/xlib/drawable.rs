use x11::xlib as x;

use xlib::{GetDisplay, Image, Rectangle, X11Error};

/// Anything that's drawable
pub trait Drawable: GetDisplay {
    /// Get drawable handle
    fn as_drawable(&self) -> x::Drawable;

    /// Capture a snapshot of this drawable, clipped by rect.
    fn get_image(&self, rect: Rectangle) -> Result<Image, X11Error> {
        let image = unsafe {
            x::XGetImage(
                self.get_display(),
                self.as_drawable(),
                rect.x as i32,
                rect.y as i32,
                rect.width,
                rect.height,
                0xffffffff,
                x::ZPixmap,
            )
        };
        Ok(Image { inner: image })
    }
}
