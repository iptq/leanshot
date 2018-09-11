//! Safe-ish bindings to parts of x11's xlib module.
//!
//! I need this for my project.

mod display;
mod drawable;
mod errors;
mod event;
mod gc;
mod image;
mod rect;
mod visual;
mod window;

pub use self::display::{Display, GetDisplay};
pub use self::drawable::Drawable;
pub use self::errors::X11Error;
pub use self::event::{Event, EventKind};
pub use self::gc::GC;
pub use self::image::{Image, ImageByteOrder, PixBuffer};
pub use self::rect::Rectangle;
pub use self::visual::Visual;
pub use self::window::{Window, WindowAttributes};
