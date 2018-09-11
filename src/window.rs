use xlib::{Display, Rectangle, Window};

use errors::ScreenshotError;

/// A window that's opened for the user to select a region
pub struct SelectWindow {
    inner: Window,
}

impl SelectWindow {
    /// Creates a new SelectWindow
    pub fn new(display: &Display) -> Result<Self, ScreenshotError> {
        // TODO: unhardcode
        let window = Window::create(display, None, Rectangle::new(0, 0, 1920, 1080))?;
        Ok(SelectWindow { inner: window })
    }
}
