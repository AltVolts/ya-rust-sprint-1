use crate::{RecordParser, TransactionRecord};
use csv::{QuoteStyle, ReaderBuilder, WriterBuilder};
use std::io::{Read, Result, Write};

#[derive(Debug, PartialEq)]
pub struct YPBankCsvRecord {
    pub records: Vec<TransactionRecord>,
}

impl RecordParser for YPBankCsvRecord {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(r);

        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: TransactionRecord = result
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
            records.push(record);
        }
        Ok(YPBankCsvRecord { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        // Отключаем автоматические кавычки
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .quote_style(QuoteStyle::Never)
            .from_writer(writer);

        // Заголовки без кавычек
        wtr.write_record(&[
            "TX_ID",
            "TX_TYPE",
            "FROM_USER_ID",
            "TO_USER_ID",
            "AMOUNT",
            "TIMESTAMP",
            "STATUS",
            "DESCRIPTION",
        ])?;

        for record in &self.records {
            let description = format!("\"{}\"", record.description);

            wtr.write_record(&[
                record.tx_id.to_string(),
                record.tx_type.to_string(),
                record.from_user_id.to_string(),
                record.to_user_id.to_string(),
                record.amount.to_string(),
                record.timestamp.to_string(),
                record.status.to_string(),
                description,
            ])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Status, TxType};
    use std::io::Cursor;

    #[test]
    fn test_read_write_csv_records() {
        let mut test_csv_records = YPBankCsvRecord {
            records: vec![TransactionRecord {
                tx_type: TxType::DEPOSIT,
                status: Status::FAILURE,
                tx_id: 1000000000000000,
                from_user_id: 0,
                to_user_id: 9223372036854775807,
                amount: 100,
                timestamp: 1633036860000,
                description: "Record number 1".to_string(),
            }],
        };

        let mut buffer = Cursor::new(Vec::new());
        test_csv_records.write_to(&mut buffer).unwrap();
        buffer.set_position(0);

        let buff_record = YPBankCsvRecord::from_read(&mut buffer).unwrap();
        assert_eq!(test_csv_records, buff_record);
    }
}
