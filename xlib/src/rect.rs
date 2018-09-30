/// A rectangle.
#[derive(Debug)]
pub struct Rectangle {
    /// x
    pub x: i32,
    /// y
    pub y: i32,
    /// width
    pub width: u32,
    /// height
    pub height: u32,
}

impl Rectangle {
    /// Create a new Rectangle from u32s
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}
