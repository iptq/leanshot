use std::ffi::CString;
use x11::xlib as x;

use Display;
use X11Error;

/// A unique string or intger
pub struct Atom {
    inner: x::Atom,
}

impl Atom {
    /// Create a new atom using a string
    pub fn new(
        display: &Display,
        val: impl AsRef<str>,
        only_if_exists: bool,
    ) -> Result<Self, X11Error> {
        let val = {
            let v = val.as_ref();
            let s = CString::new(v).unwrap();
            s.as_ptr()
        };
        let inner =
            unsafe { x::XInternAtom(display.as_raw(), val, if only_if_exists { 1 } else { 0 }) };
        Ok(Atom { inner })
    }

    /// Create a new Atom object from an existing handle
    pub fn from(handle: x::Atom) -> Self {
        Atom { inner: handle }
    }

    /// Get the handle
    pub fn as_raw(&self) -> x::Atom {
        self.inner
    }
}
