use clap::{Parser, ValueEnum};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use ya_rust_sprint_1::{
    RecordParser, TransactionRecord, YPBankBinRecords, YPBankCsvRecords, YPBankTxtRecords,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum FileFormat {
    Csv,
    Txt,
    Binary,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long = "file1")]
    file1: PathBuf,

    #[arg(long = "file2")]
    file2: PathBuf,

    #[arg(long = "format1", value_enum)]
    format1: FileFormat,

    #[arg(long = "format2", value_enum)]
    format2: FileFormat,
}

fn read_records(
    path: &PathBuf,
    format: FileFormat,
) -> Result<Vec<TransactionRecord>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let records = match format {
        FileFormat::Csv => YPBankCsvRecords::from_read(&mut file)?.records,
        FileFormat::Txt => YPBankTxtRecords::from_read(&mut file)?.records,
        FileFormat::Binary => YPBankBinRecords::from_read(&mut file)?.records,
    };
    Ok(records)
}

fn compare_records(
    records1: &[TransactionRecord],
    records2: &[TransactionRecord],
    file1: &Path,
    file2: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut map1: HashMap<u64, &TransactionRecord> = HashMap::new();
    for record in records1 {
        map1.insert(record.tx_id, record);
    }

    let mut map2: HashMap<u64, &TransactionRecord> = HashMap::new();
    for record in records2 {
        map2.insert(record.tx_id, record);
    }

    let mut has_diff = false;
    // Проверяем записи из первого файла на наличие во втором
    for (tx_id, rec1) in &map1 {
        match map2.get(tx_id) {
            Some(rec2) => {
                if rec1 != rec2 {
                    println!("Transaction {} differs:", tx_id);
                    println!("  In {}: {}", file1.display(), rec1);
                    println!("  In {}: {}", file2.display(), rec2);
                    has_diff = true;
                }
            }
            None => {
                println!(
                    "Transaction {} present in {} but missing in {}",
                    tx_id,
                    file1.display(),
                    file2.display()
                );
                has_diff = true;
            }
        }
    }

    // Проверяем записи из второго файла на наличие в первом
    for tx_id in map2.keys() {
        if !map1.contains_key(tx_id) {
            println!(
                "Transaction {} present in {} but missing in {}",
                tx_id,
                file2.display(),
                file1.display()
            );
            has_diff = true;
        }
    }

    if !has_diff {
        println!("The transaction records are identical.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let records1 = read_records(&cli.file1, cli.format1)?;
    let records2 = read_records(&cli.file2, cli.format2)?;

    compare_records(&records1, &records2, &cli.file1, &cli.file2)?;

    Ok(())
}
