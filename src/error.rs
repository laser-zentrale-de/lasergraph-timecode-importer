use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}
