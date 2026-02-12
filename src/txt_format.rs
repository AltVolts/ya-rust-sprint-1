use std::collections::HashMap;
use crate::{TransactionRecord, RecordParser, TxType};
use serde::Deserialize;
use std::io::{BufRead, BufReader, Read, Result, Write, Error, ErrorKind};

#[derive(Debug)]
pub struct YPBankTxtRecord {
    records: Vec<TransactionRecord>,
}

impl RecordParser for YPBankTxtRecord {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut records = Vec::new();
        let mut buffer = BufReader::new(r);
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

        Ok(YPBankTxtRecord { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        todo!()
    }
}

fn add_line_to_map(line: String, map: &mut HashMap<String, String>) -> Result<()> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Incorrect line format {}", line
            ),
        ));
    }
    let (k, v) = (parts[0], parts[1]);
    map.insert(k.to_string(), v.to_string());
    Ok(())
}

fn trim_line(line: Result<String>) -> Result<String>{
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

    fn take_parse<T: FromStr>(map: &mut HashMap<String, String>, key: &str) -> Result<T>
    where
        T::Err: std::fmt::Display,
    {
        let s = take_string(map, key)?;
        s.parse::<T>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Ошибка парсинга {}: {}", key, e)))
    }

    Ok(TransactionRecord {
        tx_id: take_parse(map, "tx_id")?,
        tx_type: take_parse::<TxType>(map, "tx_type")?,
        from_user_id: take_parse(map, "from_user_id")?,
        to_user_id: take_parse(map, "to_user_id")?,
        amount: take_parse(map, "amount")?,
        timestamp: take_parse(map, "timestamp")?,
        status: take_parse::<Status>(map, "status")?,
        description: take_string(map, "description")?,
    })
}