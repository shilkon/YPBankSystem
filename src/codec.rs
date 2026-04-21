mod error;
mod csv;
mod txt;

pub use self::error::CodecError;
pub use self::{csv::CsvFormat, txt::TxtFormat};

use crate::transaction::Transaction;

use enum_dispatch::enum_dispatch;

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
    *pos += 1;
        
    if bytes_read == 0 {
        return Ok(None) // EOF
    }

    Ok(Some(()))
}
