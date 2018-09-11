/// Enumerated error type.
#[allow(missing_docs)]
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "error")]
    Error,
}
