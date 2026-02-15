use std::fs::File;
use std::io::BufReader;
use ya_rust_sprint_1::{RecordParser, YPBankCsvRecords};

const TEST_FILE: &str = "./test_files/records_example.csv";
const WRITE_TEST_FILE: &str = "./write_test_files/records_example.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(TEST_FILE).unwrap();

    let mut records = YPBankCsvRecords::from_read(&mut BufReader::new(f))?;

    let mut f_write = File::create(WRITE_TEST_FILE).unwrap();
    records.write_to(&mut f_write)?;

    Ok(())
}
