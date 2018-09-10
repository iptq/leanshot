use std::ffi::CString;

use libc;
use x11::xlib::{self, *};

use errors::ScreenshotError;
use image::Image;
use Rectangle;

pub struct GUI {
    display: *mut Display,
}

impl GUI {
    pub fn new() -> Result<Self, ScreenshotError> {
        let display_str = CString::new(":0").unwrap();
        let display = unsafe { xlib::XOpenDisplay(display_str.as_ptr()) };
        if display.is_null() {
            return Err(ScreenshotError::XError {
                message: format!("failed to open display"),
            });
        }
        unsafe { XGrabServer(display) };
        Ok(GUI { display })
    }

    fn get_window_attributes(
        &self,
        window: Window,
    ) -> Result<*mut XWindowAttributes, ScreenshotError> {
        let attr_size = ::std::mem::size_of::<XWindowAttributes>();
        let attr = unsafe { libc::malloc(attr_size) as *mut XWindowAttributes };
        let result = unsafe { XGetWindowAttributes(self.display, window, attr) };
        if result == 0 {
            return Err(ScreenshotError::XError {
                message: format!("failed to get window attributes"),
            });
        }
        Ok(attr)
    }

    /// Captures the window and produces a DynamicImage.
    pub fn window_capture(&self, window: Window) -> Result<Image, ScreenshotError> {
        let attr = self.get_window_attributes(window)?;
        println!("got window attributes");
        let image = unsafe {
            xlib::XGetImage(
                self.display,
                window,
                (*attr).x,
                (*attr).y,
                (*attr).width as u32,
                (*attr).height as u32,
                0xffffffff,
                ZPixmap,
            )
        };
        Ok(Image::from(self.display, image))
    }

    /// Get the full screen.
    pub fn get_root_window(&self) -> Window {
        unsafe { xlib::XRootWindow(self.display, 0) as Window }
    }

    /// Get the active window.
    pub fn get_active_window(&self) -> Window {
        let mut window: Window = unsafe { ::std::mem::uninitialized() };
        let mut revert_to_return: i32 = unsafe { ::std::mem::uninitialized() };
        unsafe { xlib::XGetInputFocus(self.display, &mut window, &mut revert_to_return) };
        window
    }

    /// Brings up an interactive selection GUI.
    pub fn interactive_select(&self, _capture: &Image) -> Result<Rectangle, ScreenshotError> {
        Err(ScreenshotError::Error)
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        unsafe { XUngrabServer(self.display) };
    }
}
