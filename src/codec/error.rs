use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("I/O failure: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid format: {0}")]
    Format(String),

    #[error("Invalid structure")]
    InvalidStructure,
}

impl From<csv::Error> for CodecError {
    fn from(err: csv::Error) -> Self {
        match err.into_kind() {
            csv::ErrorKind::Io(e) => {
                CodecError::Io(e)
            }
            csv::ErrorKind::Deserialize { err, .. } => {
                CodecError::Format(err.to_string())
            }
            _ => CodecError::InvalidStructure,
        }
    }
}

impl From<yaml_serde::Error> for CodecError {
    fn from(err: yaml_serde::Error) -> Self {
        CodecError::Format(err.to_string())
    }
}
