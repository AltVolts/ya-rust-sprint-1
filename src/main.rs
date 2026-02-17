use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::stdout;
use ya_rust_sprint_1::{RecordParser, YPBankBinRecords, YPBankCsvRecords, YPBankTxtRecords};

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

    let mut input = File::open(cli.input)?;

    let records = match cli.input_format {
        FileFormat::Csv => YPBankCsvRecords::from_read(&mut input)?.records,
        FileFormat::Txt => YPBankTxtRecords::from_read(&mut input)?.records,
        FileFormat::Binary => YPBankBinRecords::from_read(&mut input)?.records,
    };

    match cli.output_format {
        FileFormat::Csv => {
            YPBankCsvRecords::new(records).write_to(&mut stdout())?;
        }
        FileFormat::Txt => {
            YPBankTxtRecords::new(records).write_to(&mut stdout())?;
        }
        FileFormat::Binary => {
            YPBankBinRecords::new(records).write_to(&mut stdout())?;
        }
    }
    Ok(())
}
