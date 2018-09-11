use imlib2::{self, Image as Image2, Visual};
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
    pub fn capture_window(&self, window: Window) -> Result<Image2, ScreenshotError> {
        let attr = window.get_attributes()?;
        let width = attr.get_width() as i32;
        let height = attr.get_height() as i32;
        let root = self.display.get_default_root_window()?;
        let (x, y, _) = self.display.translate_coordinates(window, 0, 0, root)?;
        imlib2::context_set_display(self.display.as_raw());
        imlib2::context_set_visual(Visual::default(self.display.as_raw(), 0));
        Image2::create_from_drawable(window, 0, x, y, width, height, true).map_err(|err| err.into())
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
