use super::{
    Transaction,
    TransactionWriter, TransactionReader,
    read_next_line,
    CodecError,
};

/// CSV-формат кодирования транзакций.
/// Реализует чтение и запись транзакции в соответствии со спецификацией YPBankCsv.
pub struct CsvFormat;

impl TransactionWriter for CsvFormat {
    fn write_header<W: std::io::Write>(&self, w: &mut W) -> Result<(), CodecError> {
        w.write_all(Transaction::csv_header().as_bytes())?;
        w.write_all( b"\n")?;
        Ok(())
    }

    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError> {
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .quote_style(csv::QuoteStyle::Never)
            .from_writer(w);
        wtr.serialize(tx)?;
        wtr.flush()?;
        Ok(())
    }
}

impl TransactionReader for CsvFormat {
    fn read_header<R: std::io::BufRead + ?Sized>(&self, r: &mut R, pos: &mut usize) -> Result<Option<()>, CodecError> {
        let mut line = String::new();
        if read_next_line(r, &mut line, pos)?.is_none() {
            return Ok(None); // EOF
        }
        if line.trim() != Transaction::csv_header() {
            return Err(CodecError::InvalidStructure)
        }

        Ok(Some(()))
    }

    fn read_next<R: std::io::BufRead + ?Sized>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError> {
        let mut line = String::new();
        let mut clean_line = line.trim();
        while clean_line.is_empty() {
            if read_next_line(r, &mut line, pos)?.is_none() {
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .quoting(false)
            .from_reader(clean_line.as_bytes());
        Ok(rdr.deserialize::<Transaction>().next().transpose()?)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write, Read};

    use crate::transaction::{TransactionStatus, TransactionType};

    use super::*;

    #[test]
    fn read() -> Result<(), CodecError> {
        let tx1 = Transaction::new(1000000000000007, TransactionType::Transfer, 9223372036854775807, 7524637015105340931,
            800, 1633037280000, TransactionStatus::Pending, "\"Record number 8\"".into());
        let tx2 = Transaction::new(1000000000000033, TransactionType::Deposit, 0, 9223372036854775807,
            3400, 1633038840000, TransactionStatus::Failure, "\"Record number 34\"".into());

        let mut buf = Cursor::new(Vec::new());
        writeln!(buf, "{}", Transaction::csv_header())?;
        writeln!(buf, "1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,800,1633037280000,PENDING,\"Record number 8\"")?;
        writeln!(buf, "1000000000000033,DEPOSIT,0,9223372036854775807,3400,1633038840000,FAILURE,\"Record number 34\"")?;

        buf.set_position(0);
        let mut pos = 0;
        CsvFormat.read_header(&mut buf, &mut pos)?;

        assert_eq!(Some(tx1), CsvFormat.read_next(&mut buf, &mut pos)?);
        assert_eq!(Some(tx2), CsvFormat.read_next(&mut buf, &mut pos)?);
        assert_eq!(None, CsvFormat.read_next(&mut buf, &mut pos)?);

        Ok(())
    }

    #[test]
    fn read_fail_tx_type() -> Result<(), CodecError> {
        let mut buf = Cursor::new(Vec::new());
        writeln!(buf, "1000000000000007,TRANSFERR,9223372036854775807,7524637015105340931,800,1633037280000,PENDING,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;

        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(ref s)) if s.contains("TRANSFERR")
        ));
        
        buf.get_mut().clear();
        buf.set_position(0);
        writeln!(buf, "1000000000000007,transfer,9223372036854775807,7524637015105340931,800,1633037280000,PENDING,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;
        
        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(ref s)) if s.contains("transfer")
        ));
        
        buf.get_mut().clear();
        buf.set_position(0);
        writeln!(buf, "1000000000000007,9223372036854775807,7524637015105340931,800,1633037280000,PENDING,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;
        
        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(_))
        ));

        Ok(())
    }

    #[test]
    fn read_fail_status() -> Result<(), CodecError> {
        let mut buf = Cursor::new(Vec::new());
        writeln!(buf, "1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,800,1633037280000,PENDIN,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;

        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(ref s)) if s.contains("PENDIN")
        ));

        buf.get_mut().clear();
        buf.set_position(0);
        writeln!(buf, "1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,800,1633037280000,pending,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;
        
        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(ref s)) if s.contains("pending")
        ));

        buf.get_mut().clear();
        buf.set_position(0);
        writeln!(buf, "1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,800,1633037280000,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;
        
        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(_))
        ));

        Ok(())
    }

    #[test]
    fn read_fail_integer() -> Result<(), CodecError> {
        let mut buf = Cursor::new(Vec::new());
        writeln!(buf, "1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,800.1,1633037280000,PENDING,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;

        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(_))
        ));

        buf.get_mut().clear();
        buf.set_position(0);
        writeln!(buf, "1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,\"800\",1633037280000,PENDING,\"Record number 8\"")?;
        buf.set_position(0);
        let mut pos = 0;
        
        assert!(matches!(
            CsvFormat.read_next(&mut buf, &mut pos),
            Err(CodecError::Format(_))
        ));

        Ok(())
    }

    #[test]
    fn write() -> Result<(), CodecError> {
        let tx1 = Transaction::new(1000000000000007, TransactionType::Transfer, 9223372036854775807, 7524637015105340931,
            800, 1633037280000, TransactionStatus::Pending, "\"Record number 8\"".into());
        let tx2 = Transaction::new(1000000000000033, TransactionType::Deposit, 0, 9223372036854775807,
            3400, 1633038840000, TransactionStatus::Failure, "\"Record number 34\"".into());

        let mut buf = Cursor::new(Vec::new());
        CsvFormat.write_header(&mut buf)?;
        assert_eq!((), CsvFormat.write_record(&mut buf, &tx1)?);
        assert_eq!((), CsvFormat.write_record(&mut buf, &tx2)?);

        buf.set_position(0);
        let mut written = String::new();
        buf.read_to_string(&mut written)?;

        let expected = Transaction::csv_header() +
            "\n1000000000000007,TRANSFER,9223372036854775807,7524637015105340931,800,1633037280000,PENDING,\"Record number 8\"" +
            "\n1000000000000033,DEPOSIT,0,9223372036854775807,3400,1633038840000,FAILURE,\"Record number 34\"\n";

        assert_eq!(expected, written);

        Ok(())
    }
}
