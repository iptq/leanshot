use x11::xlib as x;

/// Mouse pointer
pub struct Cursor {
    pub(crate) display: *mut x::Display,
    pub(crate) inner: x::Cursor,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { x::XFreeCursor(self.display, self.inner) };
    }
}
