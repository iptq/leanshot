use xlib::{Display, Image, Window};

use errors::ScreenshotError;
use Rectangle;

pub struct GUI {
    pub(crate) display: Display,
}

impl GUI {
    pub fn new() -> Result<Self, ScreenshotError> {
        let display = Display::connect(":0")?;
        Ok(GUI { display })
    }

    /// Captures the window and produces a DynamicImage.
    pub fn capture_window(&self, window: Window) -> Result<Image, ScreenshotError> {
        window.get_image().map_err(|err| err.into())
    }

    /// Get the active window.
    pub fn get_active_window(&self) -> Result<Window, ScreenshotError> {
        Ok(self.display.get_default_root_window()?)
        // let mut window: Window = self.display.get_default_root_window();
        // let mut revert_to_return: i32 = RevertToParent;
        // unsafe { XGetInputFocus(self.display, &mut window, &mut revert_to_return) };
        // unsafe { XMapRaised(self.display, window) };
        // window
    }

    /// Brings up an interactive selection GUI.
    #[allow(dead_code)]
    pub fn interactive_select(&self, _capture: &Image) -> Result<Rectangle, ScreenshotError> {
        Err(ScreenshotError::Error)
    }
}
