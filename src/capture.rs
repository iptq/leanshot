use errors::ScreenshotError;

use gui::GUI;
use options::{Options, Region};

/// The main capture routine.
pub fn capture(opt: &Options) -> Result<(), ScreenshotError> {
    let gui = GUI::new()?;

    let window_to_capture = match opt.region {
        Region::ActiveWindow => gui.get_active_window()?,
        _ => gui.display.get_default_root_window()?,
    };

    let capture = gui.capture_window(&opt, window_to_capture)?;
    capture.save_image(&opt.outfile)?;

    Ok(())
}
