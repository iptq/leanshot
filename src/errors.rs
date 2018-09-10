#[derive(Debug, Fail)]
pub enum ScreenshotError {
    #[fail(display = "error")]
    Error,
}
