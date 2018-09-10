use std::ffi::CString;

use failure::Error;
use gdk_pixbuf::{Pixbuf, PixbufExt};

use errors::ScreenshotError;
use slop_sys::{slop_options, slop_select};

pub fn select_area(pixbuf: Pixbuf) -> Result<Pixbuf, Error> {
    let xdisplay = CString::new(":0")?.as_ptr();

    let mut options = slop_options {
        quiet: 0,
        border: 1.0,
        padding: 1.0,
        tolerance: 1.0,
        highlight: 0,
        nokeyboard: 0,
        noopengl: 1,
        nodecorations: 1,
        shaders: &mut 0,
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
        xdisplay,
    };
    let selection = unsafe { slop_select(&mut options) };
    println!("{:?}", selection);

    if selection.cancelled == 1 {
        bail!(ScreenshotError);
    }
    match pixbuf.new_subpixbuf(
        selection.x.round() as i32,
        selection.y.round() as i32,
        selection.w.round() as i32,
        selection.h.round() as i32,
    ) {
        Some(pixbuf) => Ok(pixbuf),
        None => bail!(ScreenshotError),
    }
}
