mod error;
mod csv;
mod txt;
mod bin;

pub use self::error::{CodecError, ParseEnumError};
pub use self::{csv::CsvFormat, txt::TxtFormat, bin::BinFormat};

use crate::transaction::{Transaction, TransactionType, TransactionStatus};

use enum_dispatch::enum_dispatch;

/// Формат кодирования транзакций
#[enum_dispatch(TransactionWriter, TransactionReader)]
pub enum Format {
    Csv(CsvFormat),
    Txt(TxtFormat),
    Bin(BinFormat)
}

/// Трейт для потоковой записи одной транзакции.
#[enum_dispatch]
pub trait TransactionWriter {
    /// Запись заголовка при необходимости
    fn write_header<W: std::io::Write>(&self, _: &mut W) -> Result<(), CodecError> {
        Ok(())
    }

    /// Сериализация одной транзакции
    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError>;
}

/// Трейт для потокового чтения одной транзакции.
/// Последний параметр `pos` позволяет отслеживать строку/позицию чтения в читаемом объекте.
#[enum_dispatch]
pub trait TransactionReader {
    /// Чтение заголовка при необходимости
    fn read_header<R: std::io::BufRead + ?Sized>(&self, _: &mut R, _: &mut usize) -> Result<Option<()>, CodecError> {
        Ok(Some(()))
    }

    /// Десериализация одной транзакции
    fn read_next<R: std::io::BufRead + ?Sized>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError>;
}

fn read_next_line<R: std::io::BufRead + ?Sized>(r: &mut R, line: &mut String, pos: &mut usize) -> Result<Option<()>, CodecError> {
    line.clear();
    let bytes_read = r.read_line(line)?;
    *pos += 1;
        
    if bytes_read == 0 {
        return Ok(None) // EOF
    }

    Ok(Some(()))
}
