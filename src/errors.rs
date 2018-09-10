#[derive(Debug, Fail)]
pub enum ScreenshotError {
    #[fail(display = "x11 error: {}", message)]
    XError { message: String },

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

impl From<::png::EncodingError> for ScreenshotError {
    fn from(err: ::png::EncodingError) -> Self {
        ScreenshotError::PngEncodingError(err)
    }
}
