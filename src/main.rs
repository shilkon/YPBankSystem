use std::{fs::File, io::{BufRead, BufReader, Write}};

use clap::{Parser, ValueEnum};

use anyhow::Context;
use parser::{Format, CsvFormat, TxtFormat, BinFormat, TransactionReader, TransactionWriter};

#[derive(Parser)]
struct CliArgs {
    input: Option<String>,

    #[arg(long, value_enum)]
    in_format: DataFormat,

    #[arg(long, value_enum)]
    out_format: DataFormat,
}

#[derive(Clone, ValueEnum)]
enum DataFormat {
    Csv,
    Txt,
    Bin,
}

fn match_format(format: DataFormat) -> Format {
    match format {
        DataFormat::Csv => Format::Csv(CsvFormat),
        DataFormat::Txt => Format::Txt(TxtFormat),
        DataFormat::Bin => Format::Bin(BinFormat),
    }
}

fn open_input(path: &Option<String>) -> Result<Box<dyn BufRead>, std::io::Error> {
    match path {
        Some(p) => Ok(Box::new(BufReader::new(File::open(p)?))),
        None => Ok(Box::new(BufReader::new(std::io::stdin()))),
    }
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let mut input = open_input(&args.input)
        .context(format!("Failed to open input file '{}'", args.input.as_deref().unwrap_or("stdin")))?;
    let tx_reader = match_format(args.in_format);

    let mut output = std::io::stdout();
    let tx_writer = match_format(args.out_format);

    let mut position: usize = 0;
    if tx_reader.read_header(&mut *input, &mut position)
        .context(format!("Failed to read header from input'{}'", args.input.as_deref().unwrap_or("stdin")))?
        .is_none() {
        return Ok(())
    }
    
    tx_writer.write_header(&mut output).context("Failed to write header to stdout")?;

    while let Some(tx_record) = tx_reader.read_next(&mut input, &mut position).transpose() {
        match tx_record {
            Ok(tx) => {
                if let Err(e) = tx_writer.write_record(&mut output, &tx) {
                    anyhow::bail!("Failed to write transaction:\n{tx}\nError: {e}")
                }
            }
            Err(e) => anyhow::bail!("Failed to read transaction at line/position {position}: {e}")
        }
    }

    output.flush()?;

    Ok(())
}
