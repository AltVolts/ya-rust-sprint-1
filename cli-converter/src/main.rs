use anyhow::Context;
use clap::{Parser, ValueEnum};
use parser::{RecordParser, YPBankBinRecords, YPBankCsvRecords, YPBankTxtRecords};
use std::fs::File;
use std::io::stdout;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum FileFormat {
    Csv,
    Txt,
    Binary,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short = 'f', long = "input-format", value_enum)]
    input_format: FileFormat,

    #[arg(short = 'o', long = "output-format")]
    output_format: FileFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut input = File::open(&cli.input).with_context(|| {
        format!(
            "Failed to open input file '{}' specified in --input argument",
            cli.input
        )
    })?;

    let records = match cli.input_format {
        FileFormat::Csv => {
            YPBankCsvRecords::from_read(&mut input)
                .with_context(|| format!("Failed to parse as csv data from file '{}'", cli.input))?
                .records
        }
        FileFormat::Txt => {
            YPBankTxtRecords::from_read(&mut input)
                .with_context(|| format!("Failed to parse as txt data from file '{}'", cli.input))?
                .records
        }
        FileFormat::Binary => {
            YPBankBinRecords::from_read(&mut input)
                .with_context(|| {
                    format!("Failed to parse as binary data from file '{}'", cli.input)
                })?
                .records
        }
    };

    match cli.output_format {
        FileFormat::Csv => {
            YPBankCsvRecords::new(records)
                .write_to(&mut stdout())
                .with_context(|| "Failed to write output as csv records")?;
        }
        FileFormat::Txt => {
            YPBankTxtRecords::new(records)
                .write_to(&mut stdout())
                .with_context(|| "Failed to write output as txt records")?;
        }
        FileFormat::Binary => {
            YPBankBinRecords::new(records)
                .write_to(&mut stdout())
                .with_context(|| "Failed to write output as binary records")?;
        }
    }
    Ok(())
}
