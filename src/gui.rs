use imlib2::{self, Image as Image2};
use xlib::{Display, EventKind, Visual, Window};

use errors::ScreenshotError;
use Options;
use Rectangle;
use SelectWindow;
use Region;

pub struct GUI {
    pub(crate) display: Display,
}

impl GUI {
    pub fn new() -> Result<Self, ScreenshotError> {
        let display = Display::connect(":0")?;
        Ok(GUI { display })
    }

    /// Captures the window and produces an Image.
    pub fn capture_window(&self, opt: &Options, window: Window) -> Result<Image2, ScreenshotError> {
        let attr = window.get_attributes()?;
        let mut width = attr.get_width();
        let mut height = attr.get_height();
        let root = self.display.get_default_root_window()?;
        let (mut x, mut y, _) = self.display.translate_coordinates(window, 0, 0, root)?;

        imlib2::context_set_display(&self.display);
        let visual = Visual::default(&self.display, 0);
        imlib2::context_set_visual(&visual);

        match opt.region {
            Region::Selection => {
                let region = self.interactive_select()?;
                x = region.x;
                y = region.y;
                width = region.width;
                height = region.height;
            },
            _ => ()
        }

        Image2::create_from_drawable(window, 0, x, y, width as i32, height as i32, true)
            .map_err(|err| err.into())
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
    pub fn interactive_select(&self) -> Result<Rectangle, ScreenshotError> {
        let window = SelectWindow::new(&self.display);
        let root = self.display.get_default_root_window()?;

        let root_im = root.get_image();

        let mut done = 0;
        while done == 0 && self.display.pending()? > 0 {
            let ev = self.display.next_event()?;
            match ev.kind() {
                EventKind::None => (),
            }
        }
        Err(ScreenshotError::Error)
    }
}
