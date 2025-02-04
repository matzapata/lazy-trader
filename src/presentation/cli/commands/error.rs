#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{0}")]
    Io(#[from] dialoguer::Error),
    #[error("{0}")]
    InvalidArgument(String),
    #[error("{0}")]
    InternalError(String),
}

impl From<Box<dyn std::error::Error>> for CliError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        CliError::InternalError(e.to_string())
    }
}