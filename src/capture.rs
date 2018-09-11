use errors::ScreenshotError;

use gui::GUI;
use options::{Options, Region};
use xlib::Image;

pub fn capture(opt: &Options) -> Result<Image, ScreenshotError> {
    let gui = GUI::new()?;

    let window_to_capture = match opt.region {
        Region::Fullscreen | Region::Selection => gui.display.get_default_root_window()?,
        Region::ActiveWindow => gui.get_active_window()?,
    };

    let capture = gui.capture_window(window_to_capture)?;
    println!("captured the window");

    // let final_capture = match opt.region {
    //     Region::Fullscreen | Region::ActiveWindow => window_capture,
    //     Region::Selection => gui.interactive_select(&window_capture),
    // };

    // if let Region::Selection = opt.region {
    //     let region = gui.interactive_select(&capture)?;
    //     capture.apply_region(&region);
    // };

    Ok(capture)
}
