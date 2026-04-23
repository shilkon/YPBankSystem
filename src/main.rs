use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

use anyhow::Context;
use yp_bank_system::{Format, CsvFormat, TxtFormat, BinFormat, TransactionReader, TransactionWriter};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        anyhow::bail!("Usage: yp_bank_system <input_file> <output_file>");
    }

    let match_format = |path: &Path| match path.extension().and_then(|s| s.to_str()) {
        Some("csv") => Ok(Format::Csv(CsvFormat)),
        Some("txt") => Ok(Format::Txt(TxtFormat)),
        Some("bin") => Ok(Format::Bin(BinFormat)),
        _ => Err(anyhow::anyhow!("Unsupported file format: '{}'", path.display())),
    };

    let input_file_path = Path::new(&args[0]);
    let tx_reader = match_format(input_file_path)?;

    let output_file_path = Path::new(&args[1]);
    let tx_writer = match_format(output_file_path)?;

    let input_file = File::open(input_file_path)
        .context(format!("Failed to open input file '{}'", input_file_path.display()))?;

    let output_file = File::create(output_file_path)
        .context(format!("Failed to create output file '{}'", output_file_path.display()))?;

    let mut buf_reader = std::io::BufReader::new(input_file);
    let mut position: usize = 0;
    if tx_reader.read_header(&mut buf_reader, &mut position)
        .context(format!("Failed to read header from '{}'", input_file_path.display()))?.is_none() {
        return Ok(())
    }
    
    let mut buf_writer = BufWriter::new(output_file);
    tx_writer.write_header(&mut buf_writer)
        .context(format!("Failed to write header to '{}'", output_file_path.display()))?;

    while let Some(tx_record) = tx_reader.read_next(&mut buf_reader, &mut position).transpose() {
        match tx_record {
            Ok(tx) => {
                if let Err(e) = tx_writer.write_record(&mut buf_writer, &tx) {
                    anyhow::bail!("Failed to write transaction:\n{tx}\nError: {e}")
                }
            }
            Err(e) => anyhow::bail!("Failed to read transaction at line/position {position}: {e}")
        }
    }

    buf_writer.flush()?;

    Ok(())
}
