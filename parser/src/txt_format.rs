use crate::{RecordParser, Status, TransactionRecord, TxType};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Result, Write};
use std::str::FromStr;

/// Коллекция банковских записей, полученная из TXT-файла формата YP Bank.
///
/// Хранит вектор транзакций [`TransactionRecord`](crate::TransactionRecord).
/// Используется вместе с трейтом [`RecordParser`](crate::RecordParser) для чтения и записи.
#[derive(Debug, PartialEq)]
pub struct YPBankTxtRecords {
    /// Вектор записей транзакций, извлечённых из TXT.
    pub records: Vec<TransactionRecord>,
}

impl YPBankTxtRecords {
    pub fn new(records: Vec<TransactionRecord>) -> Self {
        YPBankTxtRecords { records }
    }
}

impl RecordParser for YPBankTxtRecords {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut records = Vec::new();
        let buffer = BufReader::new(r);
        let mut current_map = HashMap::new();
        for buff_line in buffer.lines() {
            let line = trim_line(buff_line)?;
            if line.starts_with("#") {
                continue;
            }
            if line.is_empty() {
                if !current_map.is_empty() {
                    let record = hashmap_to_record(&mut current_map)?;
                    records.push(record);
                }
                current_map.clear();
            } else {
                add_line_to_map(line, &mut current_map)?;
            }
        }
        if !current_map.is_empty() {
            let record = hashmap_to_record(&mut current_map)?;
            records.push(record);
            current_map.clear();
        }

        Ok(YPBankTxtRecords { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        for record in &self.records {
            write_record_to(writer, record)?;
        }
        Ok(())
    }
}

fn write_record_to<W: Write>(w: &mut W, record: &TransactionRecord) -> Result<()> {
    let record_number = record
        .description
        .split(' ')
        .next_back()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    writeln!(w, "# Record {} {}", record_number, record.tx_type)?;

    writeln!(w, "TX_ID: {}", record.tx_id)?;
    writeln!(w, "TX_TYPE: {}", record.tx_type)?;
    writeln!(w, "FROM_USER_ID: {}", record.from_user_id)?;
    writeln!(w, "TO_USER_ID: {}", record.to_user_id)?;
    writeln!(w, "AMOUNT: {}", record.amount)?;
    writeln!(w, "TIMESTAMP: {}", record.timestamp)?;
    writeln!(w, "STATUS: {}", record.status)?;
    writeln!(w, "DESCRIPTION: \"{}\"", record.description)?;

    writeln!(w)?;

    Ok(())
}

fn add_line_to_map(line: String, map: &mut HashMap<String, String>) -> Result<()> {
    let (k, v) = match line.split_once(": ") {
        Some((k, v)) => (k, v),
        None => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Неправильный формат строки: '{}'", line),
            ));
        }
    };

    let v_clean = v.replace('"', "");

    map.insert(k.to_string(), v_clean.to_string());
    Ok(())
}

fn trim_line(line: Result<String>) -> Result<String> {
    let line = line?;
    Ok(line.trim().to_string())
}

/// Преобразует накопленный HashMap в структуру TransactionRecord.
/// После успешного преобразования все использованные ключи удаляются из карты.
fn hashmap_to_record(map: &mut HashMap<String, String>) -> Result<TransactionRecord> {
    // Функция-помощник для извлечения и парсинга строки
    fn take_string(map: &mut HashMap<String, String>, key: &str) -> Result<String> {
        map.remove(key)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, format!("Отсутствует ключ: {}", key)))
    }

    fn take_parse<T>(map: &mut HashMap<String, String>, key: &str) -> Result<T>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let s = take_string(map, key)?;
        s.parse::<T>().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Ошибка парсинга '{}': {}", key, e),
            )
        })
    }

    Ok(TransactionRecord {
        tx_id: take_parse(map, "TX_ID")?,
        tx_type: take_parse::<TxType>(map, "TX_TYPE")?,
        from_user_id: take_parse(map, "FROM_USER_ID")?,
        to_user_id: take_parse(map, "TO_USER_ID")?,
        amount: take_parse(map, "AMOUNT")?,
        timestamp: take_parse(map, "TIMESTAMP")?,
        status: take_parse::<Status>(map, "STATUS")?,
        description: take_string(map, "DESCRIPTION")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_write_txt_records() {
        let mut test_txt_records = YPBankTxtRecords {
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
        test_txt_records.write_to(&mut buffer).unwrap();
        buffer.set_position(0);

        let buff_record = YPBankTxtRecords::from_read(&mut buffer).unwrap();
        assert_eq!(test_txt_records, buff_record);
    }

    #[test]
    fn test_missing_colon() {
        let data = "\
# Record 1 DEPOSIT
TX_ID: 123
TX_TYPE DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 456
AMOUNT: 1000
TIMESTAMP: 1633036860
STATUS: SUCCESS
DESCRIPTION: \"test\"

";
        let mut cursor = Cursor::new(data);
        let result = YPBankTxtRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(err.to_string().contains("Неправильный формат строки"));
    }

    #[test]
    fn test_missing_key() {
        let data = "\
# Record 1 DEPOSIT
TX_ID: 123
FROM_USER_ID: 0
TO_USER_ID: 456
AMOUNT: 1000
TIMESTAMP: 1633036860
STATUS: SUCCESS
DESCRIPTION: \"test\"

";
        // Отсутствует TX_TYPE
        let mut cursor = Cursor::new(data);
        let result = YPBankTxtRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(err.to_string().contains("Отсутствует ключ: TX_TYPE"));
    }

    #[test]
    fn test_invalid_value_parse() {
        let data = "\
# Record 1 DEPOSIT
TX_ID: 123
TX_TYPE: 5
FROM_USER_ID: 0
TO_USER_ID: 456
AMOUNT: 1000
TIMESTAMP: 1633036860
STATUS: SUCCESS
DESCRIPTION: \"test\"

";
        let mut cursor = Cursor::new(data);
        let result = YPBankTxtRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(err.to_string().contains("Ошибка парсинга 'TX_TYPE'"));
    }

    #[test]
    fn test_empty_file() {
        let data = "";
        let mut cursor = Cursor::new(data);
        let result = YPBankTxtRecords::from_read(&mut cursor);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().records.len(), 0);
    }
}
