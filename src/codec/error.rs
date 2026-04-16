use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("I/O failure: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse field {source} at line {line}")]
    ParseText {
        source: ParseTextError,
        line: usize
    },

    #[error("Invalid structure at line {0}")]
    InvalidStructure(usize),
}

#[derive(Error, Debug)]
#[error("'{field}': '{value}'")]
pub struct ParseTextError {
    field: String,
    value: String
}

impl ParseTextError {
    pub fn new(field: &str, value: &str) -> Self {
        Self {
            field: field.into(),
            value: value.into()
        }
    }
}
