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
        if inner.is_null() {
            return Err(X11Error::DisplayOpenError);
        }
        Ok(Display { inner })
    }

    /// Gets the raw X Display handle
    pub fn as_raw(&self) -> *mut x::Display {
        self.inner
    }

    /// Returns the root window for the given screen.
    pub fn get_root_window(&self, screen: i32) -> Result<Window, X11Error> {
        let inner = unsafe { x::XRootWindow(self.inner, screen) };
        if inner == 0 {
            return Err(X11Error::GetWindowError);
        }
        let window = Window {
            display: self.inner,
            inner,
        };
        Ok(window)
    }

    /// Returns the root window for the default screen.
    pub fn get_default_root_window(&self) -> Result<Window, X11Error> {
        let inner = unsafe { x::XDefaultRootWindow(self.inner) };
        if inner == 0 {
            return Err(X11Error::GetWindowError);
        }
        let window = Window {
            display: self.inner,
            inner,
        };
        Ok(window)
    }

    /// Translate coordinates relative to w1 to coordinates relative to w2.
    /// If the coordinates are contained in a mapped child of the destination window, the third return
    /// value will hold that child window.
    pub fn translate_coordinates(
        &self,
        w1: Window,
        x: i32,
        y: i32,
        w2: Window,
    ) -> Result<(i32, i32, Option<Window>), X11Error> {
        let mut rx = 0;
        let mut ry = 0;
        let mut child_return: x::Window = 0;
        let status = unsafe {
            x::XTranslateCoordinates(
                self.inner,
                w1.inner,
                w2.inner,
                x,
                y,
                &mut rx,
                &mut ry,
                &mut child_return,
            )
        };
        if status == 0 {
            return Err(X11Error::TranslateCoordinatesError);
        }
        let child = match child_return {
            0 => None,
            val => Some(Window {
                display: self.inner,
                inner: val,
            }),
        };
        Ok((rx, ry, child))
    }

    /// eturns the focus window and the current focus state.
    pub fn get_input_focus(&self) -> Result<(Window, i32), X11Error> {
        let mut focus_return: x::Window = 0;
        let mut revert_to_return = 0;
        unsafe { x::XGetInputFocus(self.inner, &mut focus_return, &mut revert_to_return) };
        let window = Window {
            display: self.inner,
            inner: focus_return,
        };
        return Ok((window, revert_to_return));
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { x::XCloseDisplay(self.inner) };
    }
}
