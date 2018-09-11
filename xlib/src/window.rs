use std::mem;

use libc;
use x11::xlib as x;

use Image;
use X11Error;

/// A wrapper around a window handle.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Window {
    pub(super) display: *mut x::Display,
    pub(super) inner: x::Window,
}

/// Window Attributes
pub struct WindowAttributes {
    pub(self) inner: *mut x::XWindowAttributes,
}

impl Window {
    /// Get window attributes.
    pub fn get_attributes(&self) -> Result<WindowAttributes, X11Error> {
        let attr = unsafe {
            libc::malloc(mem::size_of::<x::XWindowAttributes>()) as *mut x::XWindowAttributes
        };
        let result = unsafe { x::XGetWindowAttributes(self.display, self.inner, attr) };
        match result {
            0 => Err(X11Error::GetAttributesError),
            _ => Ok(WindowAttributes { inner: attr }),
        }
    }

    /// Capture a snapshot of this window.
    pub fn get_image(&self) -> Result<Image, X11Error> {
        let attr = self.get_attributes()?;
        let image = unsafe {
            x::XGetImage(
                self.display,
                self.inner,
                attr.get_x(),
                attr.get_y(),
                attr.get_width(),
                attr.get_height(),
                0xffffffff,
                x::ZPixmap,
            )
        };
        Ok(Image { inner: image })
    }
}

impl WindowAttributes {
    /// Gets the width of the window
    pub fn get_x(&self) -> i32 {
        unsafe { (*self.inner).x as i32 }
    }

    /// Gets the height of the window
    pub fn get_y(&self) -> i32 {
        unsafe { (*self.inner).y as i32 }
    }

    /// Gets the width of the window
    pub fn get_width(&self) -> u32 {
        unsafe { (*self.inner).width as u32 }
    }

    /// Gets the height of the window
    pub fn get_height(&self) -> u32 {
        unsafe { (*self.inner).height as u32 }
    }
}

impl Drop for WindowAttributes {
    fn drop(&mut self) {
        unsafe { libc::free(self.inner as *mut libc::c_void) };
    }
}
