use std::{env, fs::File, path::Path};

use anyhow::Context;
use yp_bank_system::{Format, CsvFormat, TxtFormat, BinFormat, TransactionReader};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        anyhow::bail!("Usage: comparer -- <first_file> <second_file>");
    }

    let match_format = |path: &Path| match path.extension().and_then(|s| s.to_str()) {
        Some("csv") => Ok(Format::Csv(CsvFormat)),
        Some("txt") => Ok(Format::Txt(TxtFormat)),
        Some("bin") => Ok(Format::Bin(BinFormat)),
        _ => Err(anyhow::anyhow!("Unsupported file format: '{}'", path.display())),
    };

    let first_file_name = &args[0];
    let first_file_path = Path::new(first_file_name);
    let first_tx_reader = match_format(first_file_path)?;

    let second_file_name = &args[1];
    let second_file_path = Path::new(second_file_name);
    let second_tx_reader = match_format(second_file_path)?;

    let first_file = File::open(first_file_path)
        .context(format!("Failed to open file '{}'", first_file_path.display()))?;

    let second_file = File::open(second_file_path)
        .context(format!("Failed to open file '{}'", second_file_path.display()))?;

    let mut first_buf_reader = std::io::BufReader::new(first_file);
    let mut first_position: usize = 0;
    if first_tx_reader.read_header(&mut first_buf_reader, &mut first_position)
        .context(format!("Failed to read header from '{}'", first_file_path.display()))?.is_none() {
        return Ok(())
    }

    let mut second_buf_reader = std::io::BufReader::new(second_file);
    let mut second_position: usize = 0;
    if second_tx_reader.read_header(&mut second_buf_reader, &mut second_position)
        .context(format!("Failed to read header from '{}'", second_file_path.display()))?.is_none() {
        return Ok(())
    }

    let mut are_identical = true;
    while are_identical {
        let first_tx_record = first_tx_reader.read_next(&mut first_buf_reader, &mut first_position)
            .context(format!("Failed to read transaction from '{}' at line/position {}", first_file_name, first_position))?;
        let second_tx_record = second_tx_reader.read_next(&mut second_buf_reader, &mut second_position)
            .context(format!("Failed to read transaction from '{}' at line/position {}", second_file_name, second_position))?;
        match (first_tx_record, second_tx_record) {
            (Some(first_tx), Some(second_tx)) => {
                if first_tx != second_tx {
                    are_identical = false;
                    println!("Found different transactions\n\
                        Transaction from '{}' at line/position {}:\n{}\n\
                        Transaction from '{}' at line/position {}:\n{}",
                        first_file_path.display(), first_position, first_tx,
                        second_file_path.display(), second_position, second_tx);
                }
            }
            (Some(first_tx), None) => {
                are_identical = false;
                println!("Found different transactions\n\
                        Transaction from '{}' at line/position {}:\n{}\n\
                        Transaction from '{}' at line/position {}: missed",
                        first_file_path.display(), first_position, first_tx,
                        second_file_path.display(), second_position);
            }
            (None, Some(second_tx)) => {
                are_identical = false;
                println!("Found different transactions\n\
                        Transaction from '{}' at line/position {}: missed\n\
                        Transaction from '{}' at line/position {}:\n{}",
                        first_file_path.display(), first_position,
                        second_file_path.display(), second_position, second_tx);
            }
            (None, None) => break
        }
    }

    if are_identical {
        println!("Transaction records in '{}' and '{}' are identical", first_file_name, second_file_name);
    }

    Ok(())
}
