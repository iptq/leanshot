use std::ffi::CString;

use x11::xlib::{self, *};

use errors::ScreenshotError;
use image::Image;
use Rectangle;

pub struct GUI {
    display: *mut Display,
}

impl GUI {
    pub fn new() -> Self {
        let display_str = CString::new(":0").unwrap();
        let display = unsafe { xlib::XOpenDisplay(display_str.as_ptr()) };
        unsafe { XGrabServer(display) };
        GUI { display }
    }

    fn get_window_attributes(&self, window: *mut Window) -> *mut XWindowAttributes {
        let attr: *mut XWindowAttributes = unsafe { ::std::mem::uninitialized() };
        unsafe { XGetWindowAttributes(self.display, *window, attr) };
        attr
    }

    /// Captures the window and produces a DynamicImage.
    pub fn window_capture(&self, window: *mut Window) -> Result<Image, ScreenshotError> {
        println!("Getting window attributes.");
        let attr = unsafe { *self.get_window_attributes(window) };
        println!("Got window attributes.");
        let image = unsafe {
            xlib::XGetImage(
                self.display,
                *window,
                attr.x,
                attr.y,
                attr.width as u32,
                attr.height as u32,
                0xffffffff,
                ZPixmap,
            )
        };
        Ok(Image::from(image))
    }

    /// Get the full screen.
    pub fn get_root_window(&self) -> *mut Window {
        println!("Getting root window...");
        unsafe { xlib::XRootWindow(self.display, 0) as *mut Window }
    }

    /// Get the active window.
    pub fn get_active_window(&self) -> *mut Window {
        let window: *mut Window = unsafe { ::std::mem::uninitialized() };
        let mut revert_to_return: i32 = unsafe { ::std::mem::uninitialized() };
        unsafe { xlib::XGetInputFocus(self.display, window, &mut revert_to_return) };
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
