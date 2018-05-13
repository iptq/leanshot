use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ScreenshotError;

impl fmt::Display for ScreenshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}

impl Error for ScreenshotError {}
