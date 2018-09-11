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
        let attr = window.get_attributes()?;
        let width = attr.get_width();
        let height = attr.get_height();
        let root = self.display.get_default_root_window()?;
        let (x, y, _) = self.display.translate_coordinates(window, 0, 0, root)?;
        println!("x={} y={} width={} height={}", x, y, width, height);
        window.get_image().map_err(|err| err.into())
    }

    /// Get the active window.
    pub fn get_active_window(&self) -> Result<Window, ScreenshotError> {
        self.display
            .get_input_focus()
            .map(|(window, _)| window)
            .map_err(|err| err.into())
    }

    /// Brings up an interactive selection GUI.
    #[allow(dead_code)]
    pub fn interactive_select(&self, _capture: &Image) -> Result<Rectangle, ScreenshotError> {
        Err(ScreenshotError::Error)
    }
}
