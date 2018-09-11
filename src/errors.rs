#[derive(Debug, Fail)]
pub enum ScreenshotError {
    #[fail(display = "x11 error")]
    X11Error(#[cause] ::xlib::X11Error),

    #[fail(display = "imlib2 error")]
    ImlibError(#[cause] ::imlib2::Error),

    #[fail(display = "io error")]
    IOError(#[cause] ::std::io::Error),

    #[fail(display = "png encoding error")]
    PngEncodingError(#[cause] ::png::EncodingError),

    #[fail(display = "error")]
    Error,
}

impl From<::std::io::Error> for ScreenshotError {
    fn from(err: ::std::io::Error) -> Self {
        ScreenshotError::IOError(err)
    }
}

impl From<::xlib::X11Error> for ScreenshotError {
    fn from(err: ::xlib::X11Error) -> Self {
        ScreenshotError::X11Error(err)
    }
}

impl From<::imlib2::Error> for ScreenshotError {
    fn from(err: ::imlib2::Error) -> Self {
        ScreenshotError::ImlibError(err)
    }
}

impl From<::png::EncodingError> for ScreenshotError {
    fn from(err: ::png::EncodingError) -> Self {
        ScreenshotError::PngEncodingError(err)
    }
}
