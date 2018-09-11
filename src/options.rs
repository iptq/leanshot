use std::path::PathBuf;

use ScreenshotError;

/// A region option
#[allow(missing_docs)]
pub enum Region {
    Fullscreen,
    ActiveWindow,
    Selection,
}

/// Optiosn for screenshot
#[derive(StructOpt)]
pub struct Options {
    /// The region to select
    #[structopt(parse(try_from_str = "Region::from_str"))]
    pub region: Region,

    /// The file to save the screenshot to
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    pub outfile: PathBuf,

    /// Whether or not to also copy it to the clipboard
    #[structopt(short = "c")]
    pub clipboard: bool,
}

impl Region {
    pub(self) fn from_str(x: &str) -> Result<Self, ScreenshotError> {
        match x {
            "fullscreen" => Ok(Region::Fullscreen),
            "window" => Ok(Region::ActiveWindow),
            "select" | "selection" => Ok(Region::Selection),
            _ => Err(ScreenshotError::Error),
        }
    }
}
