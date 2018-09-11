use std::ffi::CString;

use x11::xlib as x;

use Window;
use X11Error;

/// A connection to an X server.
pub struct Display {
    inner: *mut x::Display,
}

impl Display {
    /// Opens a new connection to an X server.
    ///
    /// On POSIX-conformant systems, the display name or DISPLAY environment variable can be a string in the format:
    ///     hostname:number.screen_number
    pub fn connect(display_name: impl AsRef<str>) -> Result<Display, X11Error> {
        let display_name = CString::new(display_name.as_ref()).unwrap();
        let inner = unsafe { x::XOpenDisplay(display_name.as_ptr()) };
        Ok(Display { inner })
    }

    /// Returns the root window for the given screen.
    pub fn get_root_window(&self, screen: i32) -> Result<Window, X11Error> {
        let window = Window {
            display: self.inner,
            inner: unsafe { x::XRootWindow(self.inner, screen) },
        };
        Ok(window)
    }

    /// Returns the root window for the default screen.
    pub fn get_default_root_window(&self) -> Result<Window, X11Error> {
        let window = Window {
            display: self.inner,
            inner: unsafe { x::XDefaultRootWindow(self.inner) },
        };
        Ok(window)
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { x::XCloseDisplay(self.inner) };
    }
}
