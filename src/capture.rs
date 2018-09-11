use errors::ScreenshotError;

use gui::GUI;
use options::{Options, Region};

pub fn capture(opt: &Options) -> Result<(), ScreenshotError> {
    let gui = GUI::new()?;

    // let window_to_capture = match opt.region {
    //     Region::Fullscreen | Region::Selection => gui.display.get_default_root_window()?,
    //     Region::ActiveWindow => gui.get_active_window()?,
    // };
    let window_to_capture = match opt.region {
        Region::ActiveWindow => gui.get_active_window()?,
        _ => gui.display.get_default_root_window()?,
    };

    // let _clip = match opt.region {
    //     Region::ActiveWindow => {
    //         let win = gui.get_active_window()?;
    //         let attr = win.get_attributes()?;
    //         let width = attr.get_width();
    //         let height = attr.get_height();
    //         let root = gui.display.get_default_root_window()?;
    //         let (x, y, _) = gui.display.translate_coordinates(win, 0, 0, root)?;
    //         Some(Rectangle::new(x as u32, y as u32, width, height))
    //     }
    //     _ => None,
    // };

    let capture = gui.capture_window(window_to_capture)?;

    // capture.write_png(&opt.outfile, clip)?;
    capture.save_image(&opt.outfile)?;

    // let final_capture = match opt.region {
    //     Region::Fullscreen | Region::ActiveWindow => window_capture,
    //     Region::Selection => gui.interactive_select(&window_capture),
    // };

    // if let Region::Selection = opt.region {
    //     let region = gui.interactive_select(&capture)?;
    //     capture.apply_region(&region);
    // };

    Ok(())
}
