use crate::{TransactionRecord, RecordParser};
use serde::Deserialize;
use std::io::{Read, Result, Write};
use csv::ReaderBuilder;

#[derive(Debug)]
pub struct YPBankCsvRecord {
    records: Vec<TransactionRecord>,
}

impl RecordParser for YPBankCsvRecord {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(r);

        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: TransactionRecord = result.map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
            })?;
            records.push(record);
        }
        Ok(YPBankCsvRecord { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        todo!()
    }
}
