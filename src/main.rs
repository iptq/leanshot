extern crate cairo;
extern crate clap;
#[macro_use]
extern crate failure;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate time;

mod capture;
mod errors;
mod options;
mod selection;
mod slop;

use std::process::exit;

use clap::{App, Arg};
use failure::Error;

use options::{Options, Region};

fn main() {
    match run() {
        Ok(()) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let app = App::new("screenshot")
        .version("0.1")
        .arg(
            Arg::with_name("region")
                .help("The region to be captured.")
                .takes_value(false)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .alias("o")
                .long("output")
                .help("Specify the output file.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("clip")
                .alias("c")
                .long("clip")
                .help("Copies the image to the clipboard if set."),
        )
        .get_matches();
    let region = match app.value_of("region").unwrap() {
        "fullscreen" => Region::Fullscreen,
        "active" | "window" => Region::ActiveWindow,
        "select" | "selection" => Region::Selection,
        _ => bail!("Please choose a valid region [fullscreen|active|select]"),
    };
    let path = String::from(app.value_of("output").unwrap());
    let clip = app.is_present("clip");
    let options = Options::new(region, path, clip)?;

    gdk::init();
    gtk::init()?;
    capture::capture(options)?;
    // unsafe {
    //     libscreenshot_main(region, outfile_c.as_ptr());
    // }
    // let image = capture(options)?.convert()?;
    // image.save("target/shiet.png")?;
    Ok(())
}
