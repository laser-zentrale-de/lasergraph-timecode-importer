use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_error() {
        // Test IoError variant
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let input_error = InputError::IoError(io_error);
        assert_eq!(
            input_error.to_string(),
            "IO error: File not found".to_string()
        );

        // Test ParseError variant
        let parse_error = InputError::ParseError("Invalid input".to_string());
        assert_eq!(
            parse_error.to_string(),
            "Parse error: Invalid input".to_string()
        );
    }
}
