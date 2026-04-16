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

pub struct TransactionIter<T: TransactionReader, R: std::io::Read> {
    reader: T,
    source: std::io::BufReader<R>,
    position: usize
}

impl<T: TransactionReader, R: std::io::Read> TransactionIter<T, R> {
    pub fn new(reader: T, source: R) -> Self {
        TransactionIter{
            reader,
            source: std::io::BufReader::new(source),
            position: 1
        }
    }

    pub fn read_header(&mut self) -> Result<Option<()>, CodecError> {
        self.reader.read_header(&mut self.source, &mut self.position)
    }
}

impl<T: TransactionReader, R: std::io::Read> Iterator for TransactionIter<T, R> {
    type Item = Result<Transaction, CodecError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_next(&mut self.source, &mut self.position).transpose()
    }
}
