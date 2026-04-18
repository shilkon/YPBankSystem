use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("I/O failure: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse field {0}")]
    ParseText(#[from] ParseTextFieldError),

    #[error("Invalid structure")]
    InvalidStructure,

    #[error("Missing field '{0}'")]
    MissingField(String)
}

#[derive(Error, Debug)]
#[error("'{key}': '{value}'")]
pub struct ParseTextFieldError {
    pub key: String,
    pub value: String
}

#[derive(Error, Debug)]
#[error("ParseEnumError")]
pub struct ParseEnumError;
