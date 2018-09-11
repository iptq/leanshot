extern crate failure;
extern crate screenshot;
extern crate structopt;

use failure::Error;
use screenshot::{capture, Options};
use structopt::StructOpt;

pub fn main() -> Result<(), Error> {
    let opt = Options::from_args();
    // let image = capture(&opt)?;
    capture(&opt).map(|_| ()).map_err(|err| err.into())
}
