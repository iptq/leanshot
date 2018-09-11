use libc;
use x11::xlib as x;

use xlib::X11Error;

/// An x11 Event
pub struct Event {
    inner: *mut x::XAnyEvent,
    kind: EventKind,
}

/// Type of event
pub enum EventKind {
    /// Was a button pressed
    ButtonPress,

    /// None event
    None,
}

impl Event {
    /// Returns the EventKind of this event
    pub fn kind(&self) -> &EventKind {
        &self.kind
    }

    pub(super) fn from_raw(event: *mut x::XAnyEvent) -> Result<Self, X11Error> {
        Ok(Event {
            inner: event,
            kind: EventKind::None,
        })
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { libc::free(self.inner as *mut libc::c_void) };
    }
}
