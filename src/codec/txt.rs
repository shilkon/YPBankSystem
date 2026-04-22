use std::collections::HashMap;

use super::{
    Transaction,
    TransactionWriter, TransactionReader,
    read_next_line,
    CodecError
};

pub struct TxtFormat;

impl TransactionWriter for TxtFormat {
    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError> {
        let value = serde_json::to_value(tx)?;

        if let serde_json::Value::Object(map) = value {
            for (key, val) in map {
                let formatted_val = match val {
                    serde_json::Value::String(s) => s,
                    _ => val.to_string(),
                };
                
                writeln!(w, "{}: {}", key.to_uppercase(), formatted_val)?;
            }
        }

        writeln!(w)?;
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
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let json_value = serde_json::to_value(block)?;
        let tx: Transaction = serde_json::from_value(json_value)?;
        
        Ok(Some(tx))
    }
}
