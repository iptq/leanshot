use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::{self, Encoder, HasParameters};
use x11::xlib::*;

use errors::ScreenshotError;
use Rectangle;

#[allow(dead_code)]
pub struct Image {
    display: *mut Display,
    inner: *mut XImage,
}

impl Image {
    pub fn from(display: *mut Display, inner: *mut XImage) -> Self {
        Image { inner, display }
    }

    pub fn apply_region(&mut self, _region: &Rectangle) {}

    /// Converts the image buffer into RGB(A).
    fn to_data_buf(&self) -> Vec<u8> {
        let im = unsafe { *self.inner };
        let size = 4usize * im.width as usize * im.height as usize;
        let mut buf = vec![1; size];
        let sbuf = unsafe { ::std::slice::from_raw_parts(im.data, size) }; // source buffer
        let mut sx = 0usize; // source idx
        let mut dx = 0usize; // dest idx
        if im.byte_order == LSBFirst {
            while dx < size {
                buf[dx] = sbuf[sx + 2] as u8;
                buf[dx + 1] = sbuf[sx + 1] as u8;
                buf[dx + 2] = sbuf[sx] as u8;
                buf[dx + 3] = if im.depth == 32 {
                    sbuf[sx + 3] as u8
                } else {
                    255u8
                };
                sx += 4;
                dx += 4;
            }
        }
        buf
    }

    pub fn write_png(&self, out: impl AsRef<Path>) -> Result<(), ScreenshotError> {
        let file = File::create(out.as_ref())?;
        let ref mut out = BufWriter::new(file);

        let (width, height) = unsafe { ((*self.inner).width as u32, (*self.inner).height as u32) };
        let mut encoder = Encoder::new(out, width, height);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self.to_data_buf();
        writer.write_image_data(data.as_slice())?;
        Ok(())
    }
}
