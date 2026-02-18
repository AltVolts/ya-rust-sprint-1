use std::fs::File;
use std::io::BufReader;
use parser::{RecordParser, YPBankTxtRecords};

const TEST_FILE: &str = "../test_files/records_example.txt";
const WRITE_TEST_FILE: &str = "./write_test_files/records_example.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(TEST_FILE)?;

    let mut records = YPBankTxtRecords::from_read(&mut BufReader::new(f))?;

    let mut f_write = File::create(WRITE_TEST_FILE)?;
    records.write_to(&mut f_write)?;

    Ok(())
}
