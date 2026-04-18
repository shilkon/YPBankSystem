use super::{
    Transaction, TransactionBuilder,
    TransactionWriter, TransactionReader,
    parse_text_field, read_next_line,
    CodecError, ParseTextFieldError,
};

pub struct TxtFormat;

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
        while clean_line.is_empty() || clean_line.starts_with("#") {
            if let None = read_next_line(r, &mut line, pos)? {
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let mut tx_builder = TransactionBuilder::default();

        let mut is_tx_end = false;
        while !is_tx_end {
            if !clean_line.starts_with("#") {
                let pair: Vec<&str> = clean_line.split(":").collect();
                if let [key, value] = pair.as_slice() {
                    match *key {
                        Transaction::TX_ID_NAME => tx_builder.tx_id = Some(parse_text_field(*key, *value)?),
                        Transaction::TX_TYPE_NAME => tx_builder.tx_type = Some(parse_text_field(*key, *value)?),
                        Transaction::FROM_USER_ID_NAME => tx_builder.from_user_id = Some(parse_text_field(*key, *value)?),
                        Transaction::TO_USER_ID_NAME => tx_builder.to_user_id = Some(parse_text_field(*key, *value)?),
                        Transaction::AMOUNT_NAME => tx_builder.amount = Some(parse_text_field(*key, *value)?),
                        Transaction::TIMESTAMP_NAME => tx_builder.timestamp = Some(parse_text_field(*key, *value)?),
                        Transaction::STATUS_NAME => tx_builder.status = Some(parse_text_field(*key, *value)?),
                        Transaction::DESCRIPTION_NAME => tx_builder.description = Some(value.trim().into()),
                        _ => return Err(CodecError::from(ParseTextFieldError{
                            key: key.trim().into(),
                            value: value.trim().into()
                        }))
                    }
                } else {
                    return Err(CodecError::InvalidStructure)
                }
            }

            if let None = read_next_line(r, &mut line, pos)? {
                is_tx_end = true;
            }
            clean_line = line.trim();
            if clean_line.is_empty() {
                is_tx_end = true;
            }
        }

        Ok(Some(tx_builder.build()?))
    }
}
