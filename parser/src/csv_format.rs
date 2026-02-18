use crate::{RecordParser, TransactionRecord};
use csv::{QuoteStyle, ReaderBuilder, WriterBuilder};
use std::io::{Read, Result, Write};

/// Коллекция банковских записей, полученная из CSV-файла формата YP Bank.
///
/// Хранит вектор транзакций [`TransactionRecord`](crate::TransactionRecord).
/// Используется вместе с трейтом [`RecordParser`](crate::RecordParser) для чтения и записи.
#[derive(Debug, PartialEq)]
pub struct YPBankCsvRecords {
    /// Вектор записей транзакций, извлечённых из CSV.
    pub records: Vec<TransactionRecord>,
}

impl YPBankCsvRecords {
    pub fn new(records: Vec<TransactionRecord>) -> Self {
        YPBankCsvRecords { records }
    }
}

impl RecordParser for YPBankCsvRecords {
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
        Ok(YPBankCsvRecords { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .quote_style(QuoteStyle::Never)
            .from_writer(writer);

        wtr.write_record([
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
        let mut test_csv_records = YPBankCsvRecords {
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

        let buff_record = YPBankCsvRecords::from_read(&mut buffer).unwrap();
        assert_eq!(test_csv_records, buff_record);
    }

    #[test]
    fn test_invalid_tx_type_value() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,deposit,0,9223372036854775807,100,1633036860,SUCCESS,\"Record number 1\"
";
        // TX_TYPE = "deposit" (нижний регистр) – не соответствует ожидаемому перечислению
        let mut cursor = Cursor::new(data);
        let result = YPBankCsvRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        // Сообщение об ошибке будет содержать информацию о невозможности десериализации
        assert!(err.to_string().contains("TX_TYPE") || err.to_string().contains("deposit"));
    }

    #[test]
    fn test_missing_field() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860,SUCCESS
";
        // Не хватает поля DESCRIPTION
        let mut cursor = Cursor::new(data);
        let result = YPBankCsvRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_invalid_number_format() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
not_a_number,DEPOSIT,0,9223372036854775807,100,1633036860,SUCCESS,\"test\"
";
        // TX_ID не является числом
        let mut cursor = Cursor::new(data);
        let result = YPBankCsvRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }
}
