use std::fs::File;
use std::io::BufReader;
use ya_rust_sprint_1::{RecordParser, YPBankBinRecord};

const TEST_FILE: &str = "./test_files/records_example.bin";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(TEST_FILE).unwrap();

    let record = YPBankBinRecord::from_read(&mut BufReader::new(f))?;

    println!("{:#?}", record);

    Ok(())
}
