use errors::ScreenshotError;

use gui::GUI;
use image::Image;
use options::{Options, Region};

pub fn capture(opt: &Options) -> Result<Image, ScreenshotError> {
    let gui = GUI::new()?;

    let window_to_capture = match opt.region {
        Region::Fullscreen | Region::Selection => gui.get_root_window(),
        Region::ActiveWindow => gui.get_active_window(),
    };
    println!("capturing window: {}", window_to_capture);

    let mut capture = gui.window_capture(window_to_capture)?;
    println!("captured the window");

    // let final_capture = match opt.region {
    //     Region::Fullscreen | Region::ActiveWindow => window_capture,
    //     Region::Selection => gui.interactive_select(&window_capture),
    // };

    if let Region::Selection = opt.region {
        let region = gui.interactive_select(&capture)?;
        capture.apply_region(&region);
    };

    Ok(capture)

    // let display;
    // let screen;
    // match (Display::get_default(), Screen::get_default()) {
    //     (Some(d), Some(s)) => {
    //         display = d;
    //         screen = s;
    //     }
    //     _ => {
    //         bail!("Failed to open screen and display.");
    //     }
    // }

    // display.sync();

    // // first, choose the window
    // let window_opt = match options.region {
    //     Region::Fullscreen | Region::Selection => screen.get_root_window(),
    //     Region::ActiveWindow => screen.get_active_window(),
    // };
    // let window: GdkWindow;
    // match window_opt {
    //     Some(window_) => window = window_,
    //     None => bail!("Failed to locate root window."),
    // }
    // window.process_updates(true);

    // // take a screenshot of it
    // let width: i32 = window.get_width();
    // let height: i32 = window.get_height();
    // let pixbuf: Pixbuf = match window.get_pixbuf(0, 0, width, height) {
    //     Some(pixbuf) => Ok(pixbuf),
    //     None => Err(ScreenshotError::InvalidPixbuf),
    // }?;

    // // launch selection
    // let pixbuf = match options.region {
    //     Region::Selection => select_area(pixbuf)?,
    //     _ => pixbuf,
    // };
    // let width = pixbuf.get_width();
    // let height = pixbuf.get_height();

    // // create and draw to the surface
    // let surface = match ImageSurface::create(Format::Rgb24, width, height) {
    //     Ok(surface) => Ok(surface),
    //     Err(_status) => Err(ScreenshotError::SurfaceCreateFail),
    // }?;
    // let ctx = Context::new(&surface);
    // ctx.set_source_pixbuf(&pixbuf, 0.0, 0.0);
    // ctx.paint();

    // // write surface to file

    // let now = time::now();
    // let path = time::strftime(&options.outfile.as_os_str().to_str().unwrap(), &now)?;
    // let mut file = File::create(path)?;
    // surface.write_to_png(&mut file)?;

    // if options.clipboard {
    //     match Clipboard::get_default(&display) {
    //         Some(clipboard) => {
    //             clipboard.set_image(&pixbuf);
    //             clipboard.store();
    //         }
    //         None => eprintln!("Failed to copy to the clipboard."),
    //     }
    // }
}
