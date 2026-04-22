use super::{
    Transaction,
    TransactionWriter, TransactionReader,
    read_next_line,
    CodecError,
};

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
    fn read_header<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<()>, CodecError> {
        let mut line = String::new();
        if let None = read_next_line(r, &mut line, pos)? {
            return Ok(None); // EOF
        }
        if line.trim() != Transaction::csv_header() {
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

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .quoting(false)
            .from_reader(clean_line.as_bytes());
        Ok(rdr.deserialize::<Transaction>().next().transpose()?)
    }
}
