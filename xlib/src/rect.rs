/// A rectangle.
#[derive(Debug)]
pub struct Rectangle {
    /// x
    pub x: u32,
    /// y
    pub y: u32,
    /// width
    pub width: u32,
    /// height
    pub height: u32,
}

impl Rectangle {
    /// Create a new Rectangle from u32s
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}