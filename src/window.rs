use xlib::{Display, Rectangle, Window};

use errors::ScreenshotError;

pub struct SelectWindow {
    inner: Window,
}

impl SelectWindow {
    pub fn new(display: &Display) -> Result<Self, ScreenshotError> {
        // TODO: unhardcode
        let window = Window::create(display, None, Rectangle::new(0, 0, 1920, 1080))?;
        Ok(SelectWindow { inner: window })
    }
}
