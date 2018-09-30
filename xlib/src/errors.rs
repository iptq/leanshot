/// Any error that can be raised when using this library.
#[allow(missing_docs)]
#[derive(Debug, Fail)]
pub enum X11Error {
    #[fail(display = "failed to create cursor")]
    CreateCursorError,

    #[fail(display = "failed to open display")]
    DisplayOpenError,

    #[fail(display = "failed to get attributes")]
    GetAttributesError,

    #[fail(display = "failed to get window")]
    GetWindowError,

    #[fail(display = "invalid byte order")]
    InvalidByteOrder,

    #[fail(display = "failed to translate coordinates")]
    TranslateCoordinatesError,

    #[fail(display = "error")]
    Error,
}
