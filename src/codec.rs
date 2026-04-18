mod error;
mod csv;
mod txt;

pub use self::error::{CodecError, ParseTextFieldError, ParseEnumError};
pub use self::{csv::CsvFormat, txt::TxtFormat};

use crate::transaction::{Transaction, TransactionBuilder};

use enum_dispatch::enum_dispatch;

static TRANSACTION_FIELDS_COUNT: usize = 8;

#[enum_dispatch(TransactionWriter)]
#[enum_dispatch(TransactionReader)]
pub enum Format {
    Csv(CsvFormat),
    Txt(TxtFormat)
}

#[enum_dispatch]
pub trait TransactionWriter {
    fn write_header<W: std::io::Write>(&self, _: &mut W) -> Result<(), CodecError> {
        Ok(())
    }

    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError>;
}

#[enum_dispatch]
pub trait TransactionReader {
    fn read_header<R: std::io::BufRead>(&self, _: &mut R, _: &mut usize) -> Result<Option<()>, CodecError> {
        Ok(Some(()))
    }

    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError>;
}

fn read_next_line<R: std::io::BufRead>(r: &mut R, line: &mut String, pos: &mut usize) -> Result<Option<()>, CodecError> {
    line.clear();
    let bytes_read = r.read_line(line)?;
        
    if bytes_read == 0 {
        return Ok(None) // EOF
    }

    *pos += 1;
    Ok(Some(()))
}

fn parse_text_field<F: std::str::FromStr>(key: &str, value: &str) -> Result<F, ParseTextFieldError> {
    value.trim().parse().map_err(|_| ParseTextFieldError{
        key: key.into(),
        value: value.into()
    })
}
