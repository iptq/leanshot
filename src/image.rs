use x11::xlib::XImage;

use Rectangle;

pub struct Image {
    _inner: *mut XImage,
}

impl Image {
    pub fn from(inner: *mut XImage) -> Self {
        Image { _inner: inner }
    }

    pub fn apply_region(&mut self, _region: &Rectangle) {}
}
