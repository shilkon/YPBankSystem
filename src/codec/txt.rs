use super::{
    Transaction,
    TransactionWriter, TransactionReader,
    read_next_line,
    CodecError
};

pub struct TxtFormat;

impl TransactionWriter for TxtFormat {
    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError> {
        let block = yaml_serde::to_string(tx)?;
        w.write_all( block.as_bytes())?;
        w.write_all( b"\n")?;
        Ok(())
    }
}

impl TransactionReader for TxtFormat {
    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError> {
        let mut line = String::new();
        let mut clean_line = line.trim();
        while clean_line.is_empty() {
            if let None = read_next_line(r, &mut line, pos)? {
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let mut block = String::new();

        while !clean_line.is_empty() {
            block.push_str(&line);

            if let None = read_next_line(r, &mut line, pos)? {
                return Ok(None); // EOF
            }
            clean_line = line.trim();
        }

        let tx: Transaction = yaml_serde::from_str(&block)?;
        
        Ok(Some(tx))
    }
}
