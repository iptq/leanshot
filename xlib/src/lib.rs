//! Safe-ish bindings to parts of x11's xlib module.
//!
//! I need this for my project.

#[macro_use]
extern crate failure;
extern crate libc;
pub extern crate x11;

mod atom;
mod cursor;
mod display;
mod drawable;
mod errors;
mod event;
mod image;
mod rect;
mod visual;
mod window;

#[allow(non_upper_case_globals)]
#[allow(missing_docs)]
mod cursorfont;

pub use atom::Atom;
pub use cursor::Cursor;
pub use cursorfont::*;
pub use display::{Display, GetDisplay};
pub use drawable::Drawable;
pub use errors::X11Error;
pub use event::{Event, EventKind};
pub use image::{Image, ImageByteOrder, PixBuffer};
pub use rect::Rectangle;
pub use visual::Visual;
pub use window::{Window, WindowAttributes};
