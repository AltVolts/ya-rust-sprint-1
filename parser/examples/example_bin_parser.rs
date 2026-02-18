use parser::{RecordParser, YPBankBinRecords};
use std::fs::File;
use std::io::BufReader;

const TEST_FILE: &str = "../test_files/records_example.bin";
const WRITE_TEST_FILE: &str = "./write_test_files/records_example.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(TEST_FILE)?;

    let mut records = YPBankBinRecords::from_read(&mut BufReader::new(f))?;

    let mut f_write = File::create(WRITE_TEST_FILE)?;
    records.write_to(&mut f_write)?;

    Ok(())
}
