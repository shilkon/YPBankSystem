use std::str::FromStr;

use super::{Transaction, TRANSACTION_FIELDS_COUNT, TransactionWriter, TransactionReader, CodecError, ParseTextError};

#[derive(Copy, Clone)]
pub struct CsvFormat;
// TODO: line number in error

impl CsvFormat {
    const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";
}

fn parse_field<F: FromStr>(s: &str, field: &str) -> Result<F, ParseTextError> {
    s.parse().map_err(|_| ParseTextError::new(field, s))
}

fn parse_transaction(parts: &Vec<&str>) -> Result<Transaction, ParseTextError> {
    Ok(Transaction {
        tx_id: parse_field(parts[0], "TX_ID")?,
        tx_type: parts[1].parse()?,
        from_user_id: parse_field(parts[2], "FROM_USER_ID")?,
        to_user_id: parse_field(parts[3], "TO_USER_ID")?,
        amount: parse_field(parts[4], "AMOUNT")?, 
        timestamp: parse_field(parts[5], "TIMESTAMP")?,
        status: parts[6].parse()?,
        description: parts[7].into()
    })
}

impl TransactionWriter for CsvFormat {
    fn write_header<W: std::io::Write>(&self, w: &mut W) -> Result<(), CodecError> {
        writeln!(w, "{}", Self::HEADER)?;
        Ok(())
    }

    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError> {
        writeln!(w, "{},{},{},{},{},{},{},{}",
            tx.tx_id,
            tx.tx_type,
            tx.from_user_id,
            tx.to_user_id,
            tx.amount,
            tx.timestamp,
            tx.status,
            tx.description
        )?;
        Ok(())
    }
}

impl TransactionReader for CsvFormat {
    fn read_header<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<()>, CodecError> {
        let mut line = String::new();
        let bytes_read = r.read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(None); // EOF
        }
        *pos += 1;
        if line.trim_end() != Self::HEADER {
            println!("{}", line);
            println!("{}", Self::HEADER);
            return Err(CodecError::InvalidStructure(*pos))
        }

        Ok(Some(()))
    }

    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError> {
        let mut line = String::new();

        while line.is_empty() {
            let bytes_read = r.read_line(&mut line)?;
        
            if bytes_read == 0 {
                return Ok(None); // EOF
            }

            line = line.trim().into();
            *pos += 1;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != TRANSACTION_FIELDS_COUNT {
            return Err(CodecError::InvalidStructure(*pos))
        }

        let tx = parse_transaction(&parts).map_err(|e| CodecError::ParseText { source: e, line: *pos })?;
        
        Ok(Some(tx))
    }
}
