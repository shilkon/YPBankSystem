use super::{
    Transaction, TRANSACTION_FIELDS_COUNT,
    TransactionWriter, TransactionReader,
    parse_text_field, read_next_line,
    CodecError,
};

#[derive(Copy, Clone)]
pub struct CsvFormat;

impl CsvFormat {
    const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";
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
        if let None = read_next_line(r, &mut line, pos)? {
            return Ok(None); // EOF
        }
        if line.trim() != Self::HEADER {
            println!("{}", line);
            println!("{}", Self::HEADER);
            return Err(CodecError::InvalidStructure)
        }

        Ok(Some(()))
    }

    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError> {
        let mut line = String::new();
        let mut clean_line = line.trim();
        while clean_line.is_empty() {
            if let None = read_next_line(r, &mut line, pos)? {
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let parts: Vec<&str> = clean_line.split(',').collect();
        if parts.len() != TRANSACTION_FIELDS_COUNT {
            return Err(CodecError::InvalidStructure)
        }

        let tx = Transaction {
            tx_id: parse_text_field(Transaction::TX_ID_NAME, parts[0])?,
            tx_type: parse_text_field(Transaction::TX_TYPE_NAME, parts[1])?,
            from_user_id: parse_text_field(Transaction::FROM_USER_ID_NAME, parts[2])?,
            to_user_id: parse_text_field(Transaction::TO_USER_ID_NAME, parts[3])?,
            amount: parse_text_field(Transaction::AMOUNT_NAME, parts[4])?, 
            timestamp: parse_text_field(Transaction::TIMESTAMP_NAME, parts[5])?,
            status: parse_text_field(Transaction::STATUS_NAME, parts[6])?,
            description: parts[7].trim().into()
        };
        
        Ok(Some(tx))
    }
}
