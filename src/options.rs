use std::path::PathBuf;

use ScreenshotError;

pub enum Region {
    Fullscreen,
    ActiveWindow,
    Selection,
}

#[derive(StructOpt)]
pub struct Options {
    #[structopt(parse(try_from_str = "Region::from_str"))]
    pub region: Region,

    #[structopt(short = "o", long = "out", parse(from_os_str))]
    pub outfile: PathBuf,

    #[structopt(short = "c")]
    pub clipboard: bool,
}

impl Region {
    pub fn from_str(x: &str) -> Result<Self, ScreenshotError> {
        match x {
            "fullscreen" => Ok(Region::Fullscreen),
            "window" => Ok(Region::ActiveWindow),
            "select" | "selection" => Ok(Region::Selection),
            _ => Err(ScreenshotError::Error),
        }
    }
}
