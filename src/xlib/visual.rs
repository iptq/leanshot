use x11::xlib as x;

use xlib::Display;

/// A wrapper around a Visual
pub struct Visual {
    pub(super) inner: *mut x::Visual,
}

impl Visual {
    /// Gets the raw handle to the x11 Visual
    pub fn as_raw(&self) -> *mut x::Visual {
        self.inner
    }

    /// Gets the default visual
    pub fn default(display: &Display, screen: i32) -> Self {
        let inner = unsafe { x::XDefaultVisual(display.as_raw(), screen) };
        Visual { inner }
    }
}
