use std::collections::HashMap;

use super::{
    Transaction,
    TransactionWriter, TransactionReader,
    read_next_line,
    CodecError
};

pub struct TxtFormat;

fn parse_field<T: std::str::FromStr>(map: &HashMap<String, String>, key: &str) -> Result<T, CodecError> {
    let value = map.get(key)
        .ok_or_else(|| CodecError::Format(format!("Missing field: {}", key)))?;
    value.parse()
        .map_err(|_| CodecError::Format(format!("Invalid field '{}': '{}'", key, value)))
}

fn parse_string(map: &HashMap<String, String>, key: &str) -> Result<String, CodecError> {
    Ok(map.get(key)
        .ok_or_else(|| CodecError::Format(format!("Missing field: {}", key)))?
        .clone())
}

fn parse_transaction(map: &HashMap<String, String>) -> Result<Transaction, CodecError> {
    Ok(Transaction::new(
        parse_field(map, Transaction::TX_ID_NAME)?,
        parse_field(map, Transaction::TX_TYPE_NAME)?,
        parse_field(map, Transaction::FROM_USER_ID_NAME)?,
        parse_field(map, Transaction::TO_USER_ID_NAME)?,
        parse_field(map, Transaction::AMOUNT_NAME)?,
        parse_field(map, Transaction::TIMESTAMP_NAME)?,
        parse_field(map, Transaction::STATUS_NAME)?,
        parse_string(map, Transaction::DESCRIPTION_NAME)?
    ))
}

impl TransactionWriter for TxtFormat {
    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError> {
        writeln!(w, "{tx}\n")?;
        Ok(())
    }
}

impl TransactionReader for TxtFormat {
    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError> {
        let mut line = String::new();
        let mut clean_line = line.trim();
        while clean_line.is_empty() || clean_line.starts_with('#') {
            if let None = read_next_line(r, &mut line, pos)? {
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let mut block = HashMap::new();

        while !clean_line.is_empty() {
            if !clean_line.starts_with('#') {
                if let Some((key, value)) = clean_line.split_once(':') {
                    block.insert(key.trim().to_string(), value.trim().to_string());
                }
            }

            if let None = read_next_line(r, &mut line, pos)? {
                break;
            }
            clean_line = line.trim();
        }

        Ok(Some(parse_transaction(&block)?))
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write, Read};

    use crate::transaction::{TransactionStatus, TransactionType};

    use super::*;

    #[test]
    fn read() -> Result<(), CodecError> {
        let tx1 = Transaction::new(1000000000000000, TransactionType::Deposit, 0, 9223372036854775807,
            100, 1633036860000, TransactionStatus::Failure, "\"Record number 1\"".into());
        let tx2 = Transaction::new(1000000000000001, TransactionType::Transfer, 9223372036854775807, 9223372036854775807,
            200, 1633036920000, TransactionStatus::Pending, "\"Record number 2\"".into());

        let data = "# Record 1 (DEPOSIT)\n \
                          TX_TYPE: DEPOSIT\n \
                          TO_USER_ID: 9223372036854775807\n \
                          FROM_USER_ID: 0\n \
                          TIMESTAMP: 1633036860000\n \
                          DESCRIPTION: \"Record number 1\"\n \
                          TX_ID: 1000000000000000\n \
                          AMOUNT: 100\n \
                          STATUS: FAILURE\n\n \
                          # Record 2 (TRANSFER)\n \
                          DESCRIPTION: \"Record number 2\"\n \
                          TIMESTAMP: 1633036920000\n \
                          STATUS: PENDING\n \
                          AMOUNT: 200\n \
                          TX_ID: 1000000000000001\n \
                          TX_TYPE: TRANSFER\n \
                          FROM_USER_ID: 9223372036854775807\n \
                          TO_USER_ID: 9223372036854775807\n";
        
        let mut buf = Cursor::new(Vec::new());
        writeln!(buf, "{data}")?;

        buf.set_position(0);
        let mut pos = 0;
        TxtFormat.read_header(&mut buf, &mut pos)?;

        assert_eq!(Some(tx1), TxtFormat.read_next(&mut buf, &mut pos)?);
        assert_eq!(Some(tx2), TxtFormat.read_next(&mut buf, &mut pos)?);
        assert_eq!(None, TxtFormat.read_next(&mut buf, &mut pos)?);

        Ok(())
    }

    #[test]
    fn write() -> Result<(), CodecError> {
        let tx1 = Transaction::new(1000000000000000, TransactionType::Deposit, 0, 9223372036854775807,
            100, 1633036860000, TransactionStatus::Failure, "\"Record number 1\"".into());
        let tx2 = Transaction::new(1000000000000001, TransactionType::Transfer, 9223372036854775807, 9223372036854775807,
            200, 1633036920000, TransactionStatus::Pending, "\"Record number 2\"".into());

        let mut buf = Cursor::new(Vec::new());
        TxtFormat.write_header(&mut buf)?;
        assert_eq!((), TxtFormat.write_record(&mut buf, &tx1)?);
        assert_eq!((), TxtFormat.write_record(&mut buf, &tx2)?);

        buf.set_position(0);
        let mut written = String::new();
        buf.read_to_string(&mut written)?;

        let expected = "TX_ID: 1000000000000000\n\
                              TX_TYPE: DEPOSIT\n\
                              FROM_USER_ID: 0\n\
                              TO_USER_ID: 9223372036854775807\n\
                              AMOUNT: 100\n\
                              TIMESTAMP: 1633036860000\n\
                              STATUS: FAILURE\n\
                              DESCRIPTION: \"Record number 1\"\n\n\
                              TX_ID: 1000000000000001\n\
                              TX_TYPE: TRANSFER\n\
                              FROM_USER_ID: 9223372036854775807\n\
                              TO_USER_ID: 9223372036854775807\n\
                              AMOUNT: 200\n\
                              TIMESTAMP: 1633036920000\n\
                              STATUS: PENDING\n\
                              DESCRIPTION: \"Record number 2\"\n\n";

        assert_eq!(expected, written);

        Ok(())
    }
}
