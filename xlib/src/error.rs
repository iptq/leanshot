/// Any error that can be raised when using this library.
#[allow(missing_docs)]
#[derive(Debug, Fail)]
pub enum X11Error {
    #[fail(display = "failed to get attributes")]
    GetAttributesError,

    #[fail(display = "invalid byte order")]
    InvalidByteOrder,

    #[fail(display = "error")]
    Error,
}
