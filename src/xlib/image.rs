use x11::xlib as x;

use xlib::X11Error;

/// A handle to an XImage.
pub struct Image {
    pub(super) inner: *mut x::XImage,
}

/// Image byte order
pub enum ImageByteOrder {
    /// Least significant byte first
    LSBFirst,
    /// Most significant byte first
    MSBFirst,
}

/// The buffer pointed to by an XImage.
pub struct PixBuffer {
    pub(self) buf: *mut u8,
    pub(self) size: usize,
}

impl Image {
    /// Get the size (in bytes) of the data buffer.
    pub fn get_size(&self) -> usize {
        4 * self.get_width() as usize * self.get_height() as usize
    }

    /// Get the image width
    pub fn get_width(&self) -> u32 {
        unsafe { (*self.inner).width as u32 }
    }

    /// Get the image height
    pub fn get_height(&self) -> u32 {
        unsafe { (*self.inner).height as u32 }
    }

    /// Get the image depth
    pub fn get_depth(&self) -> u32 {
        unsafe { (*self.inner).depth as u32 }
    }

    /// Get byte order
    pub fn get_byte_order(&self) -> Result<ImageByteOrder, X11Error> {
        let byte_order = unsafe { (*self.inner).byte_order };
        match byte_order {
            x::LSBFirst => Ok(ImageByteOrder::LSBFirst),
            x::MSBFirst => Ok(ImageByteOrder::MSBFirst),
            _ => Err(X11Error::InvalidByteOrder),
        }
    }

    /// Produces a PixBuffer
    pub fn buffer(&self) -> PixBuffer {
        let size = self.get_size();
        let buf = unsafe { (*self.inner).data as *mut u8 };
        PixBuffer { buf, size }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { x::XDestroyImage(self.inner) };
    }
}

impl PixBuffer {
    /// Gets the byte at the index of the data buffer.
    pub fn get_byte(&self, index: usize) -> Option<u8> {
        if index > self.size {
            return None;
        }
        Some(unsafe { *self.buf.offset(index as isize) as u8 })
    }
}
