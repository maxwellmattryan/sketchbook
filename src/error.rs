/// Generic result type when using `sketchbook`.
pub type SketchbookResult<T> = std::result::Result<T, SketchbookError>;

/// Error variants when using `sketchbook`.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SketchbookError {
    #[error("Unable to run sketchbook command")]
    CannotRunCommand,

    #[error("The sketch name \"{0}\" is invalid")]
    InvalidSketchName(String),
}
