use std::mem;

// use imlib2::Drawable;
use libc;
use x11::xlib as x;

use xlib::{Display, Drawable, GetDisplay, Image, Rectangle, X11Error};

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
    /// Create a new window
    pub fn create(
        display: &Display,
        parent: Option<Window>,
        location: Rectangle,
    ) -> Result<Window, X11Error> {
        let parent = match parent {
            Some(parent) => parent,
            None => display.get_default_root_window()?,
        };
        let visual = display.default_visual(0);
        let window = unsafe {
            x::XCreateWindow(
                display.as_raw(),
                parent.as_raw(),
                location.x as i32,
                location.y as i32,
                location.width,
                location.height,
                0,
                0,
                0,
                visual.as_raw(),
                0,
                0 as *mut x::XSetWindowAttributes,
            )
        };
        Ok(Window {
            display: display.as_raw(),
            inner: window,
        })
    }

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

    /// Get the raw window handle
    pub fn as_raw(&self) -> x::Window {
        self.inner
    }

    /// Get image
    pub fn get_image(&self) -> Result<Image, X11Error> {
        let attr = self.get_attributes()?;
        Drawable::get_image(
            self,
            Rectangle::new(
                attr.get_x(),
                attr.get_y(),
                attr.get_width(),
                attr.get_height(),
            ),
        )
    }
}

impl GetDisplay for Window {
    fn get_display(&self) -> *mut x::Display {
        self.display
    }
}

impl Drawable for Window {
    fn as_drawable(&self) -> x::Drawable {
        self.inner
    }
}

// impl AsRef<Drawable> for Window {
//     fn as_ref(&self) -> &Drawable {
//         &self.inner
//     }
// }

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
