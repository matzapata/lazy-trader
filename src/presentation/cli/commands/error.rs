
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{0}")]
    Io(#[from] dialoguer::Error),
    #[error("{0}")]
    InvalidArgument(String),
}