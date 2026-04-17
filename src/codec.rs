mod error;
mod csv;

pub use self::error::{CodecError, ParseTextError};
pub use self::csv::CsvFormat;

use crate::transaction::Transaction;

static TRANSACTION_FIELDS_COUNT: usize = 8;

pub trait TransactionWriter {
    fn write_header<W: std::io::Write>(&self, _: &mut W) -> Result<(), CodecError> {
        Ok(())
    }

    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError>;
}

pub trait TransactionReader {
    fn read_header<R: std::io::BufRead>(&self, _: &mut R, _: &mut usize) -> Result<Option<()>, CodecError> {
        Ok(Some(()))
    }

    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError>;
}
