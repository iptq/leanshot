use std::fs::File;

use cairo::{Context, Format, ImageSurface};
use failure::Error;
use gdk::prelude::*;
use gdk::{ContextExt, Display, DisplayExt, Screen, ScreenExt, Window as GdkWindow, WindowExt};
use gdk_pixbuf::Pixbuf;
use gtk::{Clipboard, ClipboardExt};
use time;

use errors::ScreenshotError;
use options::{Options, Region};
use selection::select_area;

pub fn capture(options: Options) -> Result<(), Error> {
    let display;
    let screen;
    match (Display::get_default(), Screen::get_default()) {
        (Some(d), Some(s)) => {
            display = d;
            screen = s;
        }
        _ => {
            bail!("Failed to open screen and display.");
        }
    }

    display.sync();

    // first, choose the window
    let window_opt = match options.region {
        Region::Fullscreen | Region::Selection => screen.get_root_window(),
        Region::ActiveWindow => screen.get_active_window(),
    };
    let window: GdkWindow;
    match window_opt {
        Some(window_) => window = window_,
        None => bail!("Failed to locate root window."),
    }
    window.process_updates(true);

    // take a screenshot of it
    let width: i32 = window.get_width();
    let height: i32 = window.get_height();
    let pixbuf: Pixbuf = match window.get_pixbuf(0, 0, width, height) {
        Some(pixbuf) => Ok(pixbuf),
        None => Err(ScreenshotError),
    }?;

    // launch selection
    // let pixbuf = match options.region {
    //     Region::Selection => select_area(pixbuf)?,
    //     _ => pixbuf,
    // };

    // create and draw to the surface
    let surface = match ImageSurface::create(Format::Rgb24, width, height) {
        Ok(surface) => Ok(surface),
        Err(_status) => Err(ScreenshotError),
    }?;
    let ctx = Context::new(&surface);
    ctx.set_source_pixbuf(&pixbuf, 0.0, 0.0);
    ctx.paint();

    // write surface to file

    let now = time::now();
    let path = time::strftime(&options.outfile.as_os_str().to_str().unwrap(), &now)?;
    let mut file = File::create(path)?;
    surface.write_to_png(&mut file)?;

    if options.clip {
        match Clipboard::get_default(&display) {
            Some(clipboard) => {
                clipboard.set_image(&pixbuf);
            }
            None => (),
        }
    }
    Ok(())
}
