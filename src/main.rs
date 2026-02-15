use std::fs::File;
use clap::{Parser, ValueEnum};

use ya_rust_sprint_1::{RecordParser, YPBankBinRecords, YPBankCsvRecords, YPBankTxtRecords};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputFormat {
    Csv,
    Txt,
    Bin,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short = 'f', long = "input-format", value_enum)]
    input_format: InputFormat,

    // #[arg(short = 'o', long = "output-format")]
    // output_format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // println!("Входной файл: {}", cli.input);
    // println!("Формат входного файла: {}", cli.input_format);
    // println!("Формат выходного файла: {}", cli.output_format);

    let mut input = File::open(cli.input).unwrap();

    match cli.input_format {
        InputFormat::Csv => {
            let records = YPBankCsvRecords::from_read(&mut input)?;
            println!("{:#?}", records)
        }
        _ => {}
    }

    Ok(())
}
