use imlib2_sys as im;
use x11;

/// Visual
pub struct Visual {
    pub(crate) inner: *mut im::Visual,
}

impl Visual {
    /// get the default visual for a display
    pub fn default(display: *mut x11::xlib::_XDisplay, screen: i32) -> Self {
        let inner = unsafe { im::XDefaultVisual(display as *mut im::_XDisplay, screen) };
        Visual { inner }
    }
}
