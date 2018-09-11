use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::{self, Encoder, HasParameters};
use xlib::{Image, ImageByteOrder};

use errors::ScreenshotError;

pub trait ImageExt {
    fn to_data_buf(&self) -> Result<Vec<u8>, ScreenshotError>;
    fn write_png(&self, out: impl AsRef<Path>) -> Result<(), ScreenshotError>;
}

impl ImageExt for Image {
    /// Converts the image buffer into RGB(A).
    fn to_data_buf(&self) -> Result<Vec<u8>, ScreenshotError> {
        let size = self.get_size();
        let mut buf = vec![1; size];
        let sbuf = self.buffer(); // source buffer
        let mut sx = 0usize; // source idx
        let mut dx = 0usize; // dest idx
        if let ImageByteOrder::LSBFirst = self.get_byte_order()? {
            // LSBFirst
            while dx < size {
                buf[dx] = sbuf.get_byte(sx + 2).unwrap() as u8;
                buf[dx + 1] = sbuf.get_byte(sx + 1).unwrap() as u8;
                buf[dx + 2] = sbuf.get_byte(sx).unwrap() as u8;
                buf[dx + 3] = if self.get_depth() == 32 {
                    sbuf.get_byte(sx + 3).unwrap() as u8
                } else {
                    255u8
                };
                sx += 4;
                dx += 4;
            }
        }
        Ok(buf)
    }

    fn write_png(&self, out: impl AsRef<Path>) -> Result<(), ScreenshotError> {
        let file = File::create(out.as_ref())?;
        let ref mut out = BufWriter::new(file);

        let mut encoder = Encoder::new(out, self.get_width(), self.get_height());
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self.to_data_buf()?;
        writer.write_image_data(data.as_slice())?;
        Ok(())
    }
}
