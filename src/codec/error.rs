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
            csv::ErrorKind::Io(e) => CodecError::Io(e),
            csv::ErrorKind::Deserialize { err, .. } => {
                CodecError::Format(err.to_string())
            }
            _ => CodecError::InvalidStructure,
        }
    }
}

impl From<serde_json::Error> for CodecError {
    fn from(err: serde_json::Error) -> Self {
        CodecError::Format(err.to_string())
    }
}

impl From<binrw::Error> for CodecError {
    fn from(err: binrw::Error) -> Self {
        match err {
            binrw::Error::Io(e) => CodecError::Io(e),
            _ => CodecError::Format(err.to_string())
        }
    }
}

impl From<std::string::FromUtf8Error> for CodecError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        CodecError::Format(err.to_string())
    }
}
