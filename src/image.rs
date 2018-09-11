use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::{self, Encoder, HasParameters};
use xlib::{Image, ImageByteOrder};

use errors::ScreenshotError;
use Rectangle;

pub trait ImageExt {
    fn to_data_buf(&self, rect: Rectangle) -> Result<Vec<u8>, ScreenshotError>;
    fn write_png(
        &self,
        out: impl AsRef<Path>,
        rect: Option<Rectangle>,
    ) -> Result<(), ScreenshotError>;
}

impl ImageExt for Image {
    /// Converts the image buffer into RGB(A).
    fn to_data_buf(&self, rect: Rectangle) -> Result<Vec<u8>, ScreenshotError> {
        let (full_width, _) = (self.get_width() as usize, self.get_height() as usize);
        let bytes_per_row = 4 * rect.width as usize;
        let size = bytes_per_row * rect.height as usize;
        let mut buf = vec![1; size];
        let sbuf = self.buffer(); // source buffer
        let mut rx = rect.y as usize;
        if let ImageByteOrder::LSBFirst = self.get_byte_order()? {
            // LSBFirst
            while rx < rect.height as usize {
                let mut sx = (rx * full_width + rect.x as usize) * 4; // source idx
                let mut dx = rx * bytes_per_row; // dest idx
                let edx = (rx + 1) * bytes_per_row;
                // println!("sx={} dx={} edx={}", sx,dx,edx);
                while dx < edx {
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
                rx += 1;
            }
        }
        Ok(buf)
    }

    fn write_png(
        &self,
        out: impl AsRef<Path>,
        rect: Option<Rectangle>,
    ) -> Result<(), ScreenshotError> {
        let rect = rect.unwrap_or_else(|| Rectangle {
            x: 0,
            y: 0,
            width: self.get_width(),
            height: self.get_height(),
        });

        let file = File::create(out.as_ref())?;
        let ref mut out = BufWriter::new(file);

        let mut encoder = Encoder::new(out, rect.width, rect.height);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self.to_data_buf(rect)?;
        writer.write_image_data(data.as_slice())?;
        Ok(())
    }
}
