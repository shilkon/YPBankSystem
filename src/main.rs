use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

use anyhow::Context;
use yp_bank_system::{CsvFormat, TransactionReader, TransactionWriter};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        anyhow::bail!("Usage: cargo run -- <input_file> <output_file>");
    }

    let input_file_path = Path::new(&args[0]);
    let tx_reader = match input_file_path.extension().and_then(|s| s.to_str()) {
        Some("csv") => CsvFormat,
        _ => anyhow::bail!("Unsupported input file format")
    };

    let output_file_path = Path::new(&args[1]);
    let tx_writer = match output_file_path.extension().and_then(|s| s.to_str()) {
        Some("csv") => CsvFormat,
        _ => anyhow::bail!("Unsupported output file format")
    };

    let Ok(input_file) = File::open(input_file_path) else {
        anyhow::bail!("Failed to open input file: {}", input_file_path.display().to_string());
    };

    let Ok(output_file) = File::create(output_file_path) else {
        anyhow::bail!("Failed to open output file: {}", output_file_path.display().to_string());
    };

    let mut buf_reader = std::io::BufReader::new(input_file);
    let mut position: usize = 0;
    if let None = tx_reader.read_header(&mut buf_reader, &mut position).context("Failed to read header")? {
        return Ok(())
    }
    
    let mut buf_writer = BufWriter::new(output_file);
    tx_writer.write_header(&mut buf_writer).context("Failed to write header")?;

    while let Some(tx_record) = tx_reader.read_next(&mut buf_reader, &mut position).transpose() {
        match tx_record {
            Ok(tx) => {
                if let Err(e) = tx_writer.write_record(&mut buf_writer, &tx) {
                    anyhow::bail!("Failed to write transaction: {tx}\nError: {e}")
                }
            }
            Err(e) => anyhow::bail!("Failed to read transaction: {e}")
        }
    }

    buf_writer.flush()?;

    Ok(())
}
