use std::fs::{canonicalize, OpenOptions};
use std::path::PathBuf;

use failure::Error;

pub enum Region {
    Fullscreen,
    ActiveWindow,
    Selection,
}

pub struct Options {
    pub region: Region,
    pub outfile: PathBuf,
    pub clip: bool,
}

impl Options {
    pub fn new(region: Region, outfile: String, clip: bool) -> Result<Options, Error> {
        OpenOptions::new().create(true).write(true).open(&outfile)?;
        let outfile = canonicalize(&outfile)?;
        Ok(Options {
            region,
            outfile,
            clip,
        })
    }
}
