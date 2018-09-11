use errors::ScreenshotError;

use gui::GUI;
use options::{Options, Region};
use xlib::Image;

use ImageExt;
use Rectangle;

pub fn capture(opt: &Options) -> Result<Image, ScreenshotError> {
    let gui = GUI::new()?;

    // let window_to_capture = match opt.region {
    //     Region::Fullscreen | Region::Selection => gui.display.get_default_root_window()?,
    //     Region::ActiveWindow => gui.get_active_window()?,
    // };
    let window_to_capture = gui.display.get_default_root_window()?;

    let capture = gui.capture_window(window_to_capture)?;
    let clip = match opt.region {
        Region::ActiveWindow => {
            let win = gui.get_active_window()?;
            let attr = win.get_attributes()?;
            let width = attr.get_width();
            let height = attr.get_height();
            let root = gui.display.get_default_root_window()?;
            let (x, y, _) = gui.display.translate_coordinates(win, 0, 0, root)?;
            Some(Rectangle {
                x: x as u32,
                y: y as u32,
                width,
                height,
            })
        }
        _ => None,
    };

    capture.write_png(&opt.outfile, clip)?;

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
