use std::path::PathBuf;

#[derive(StructOpt)]
pub enum Region {
    #[structopt(name = "fullscreen")]
    Fullscreen,

    #[structopt(name = "window")]
    ActiveWindow,

    #[structopt(name = "select")]
    Selection,
}

#[derive(StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    pub region: Region,

    #[structopt(short = "o", long = "out", parse(from_os_str))]
    pub outfile: PathBuf,

    #[structopt(short = "c")]
    pub clipboard: bool,
}
