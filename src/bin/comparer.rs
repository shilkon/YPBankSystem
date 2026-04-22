use std::{env, fs::File, path::Path};

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
        _ => Err(anyhow::anyhow!("Unsupported file format: '{}'", path.display().to_string())),
    };

    let first_file_name = &args[0];
    let first_file_path = Path::new(first_file_name);
    let first_tx_reader = match_format(first_file_path)?;

    let second_file_name = &args[1];
    let second_file_path = Path::new(second_file_name);
    let second_tx_reader = match_format(second_file_path)?;

    let first_file = File::open(first_file_path)
        .map_err(|e| anyhow::anyhow!("Failed to open file '{}' : {}", first_file_path.display().to_string(), e))?;

    let second_file = File::open(second_file_path)
        .map_err(|e| anyhow::anyhow!("Failed to open file '{}' : {}", second_file_path.display().to_string(), e))?;

    let mut first_buf_reader = std::io::BufReader::new(first_file);
    let mut first_position: usize = 0;
    if let None = first_tx_reader.read_header(&mut first_buf_reader, &mut first_position)
        .map_err(|e| anyhow::anyhow!("Failed to read header from '{}' : {}", first_file_path.display().to_string(), e))? {
        return Ok(())
    }

    let mut second_buf_reader = std::io::BufReader::new(second_file);
    let mut second_position: usize = 0;
    if let None = second_tx_reader.read_header(&mut second_buf_reader, &mut second_position)
        .map_err(|e| anyhow::anyhow!("Failed to read header from '{}' : {}", second_file_path.display().to_string(), e))? {
        return Ok(())
    }

    let mut are_identical = true;
    while are_identical {
        let first_tx_record = first_tx_reader.read_next(&mut first_buf_reader, &mut first_position)
            .map_err(|e| anyhow::anyhow!(
                "Failed to read transaction from {first_file_name} at line/position {first_position} : {e}"
            ))?;
        let second_tx_record = second_tx_reader.read_next(&mut second_buf_reader, &mut second_position)
            .map_err(|e| anyhow::anyhow!(
                "Failed to read transaction from {second_file_name} at line/position {second_position} : {e}"
            ))?;
        match (first_tx_record, second_tx_record) {
            (Some(first_tx), Some(second_tx)) => {
                if first_tx != second_tx {
                    are_identical = false;
                    println!("Found different transactions\n\
                        Transaction from '{}' at line/position {}: {}\n\
                        Transaction from '{}' at line/position {}: {}",
                        first_file_path.display().to_string(), first_position, first_tx,
                        second_file_path.display().to_string(), second_position, second_tx);
                }
            }
            (Some(first_tx), None) => {
                are_identical = false;
                println!("Found different transactions\n\
                        Transaction from '{}' at line/position {}: {}\n\
                        Transaction from '{}' at line/position {}: missed",
                        first_file_path.display().to_string(), first_position, first_tx,
                        second_file_path.display().to_string(), second_position);
            }
            (None, Some(second_tx)) => {
                are_identical = false;
                println!("Found different transactions\n\
                        Transaction from '{}' at lin/positione {}: missed\n\
                        Transaction from '{}' at line/position {}: {}",
                        first_file_path.display().to_string(), first_position,
                        second_file_path.display().to_string(), second_position, second_tx);
            }
            (None, None) => break
        }
    }

    if are_identical {
        println!("Transaction recodrs in '{}' and '{}' are identical", first_file_name, second_file_name);
    }

    Ok(())
}
