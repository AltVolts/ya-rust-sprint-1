use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Write};

use crate::error::{BinToTransError, TransToBinError};
use crate::{RecordParser, Status, TransactionRecord, TxType};

// Постоянное значение 0x59 0x50 0x42 0x4E ('YPBN'), идентифицирующее заголовок записи.
const MAGIC: u32 = 0x5950424E;

// Размер фиксированной части записи в бинарном формате
const BODY_FIXED_PART_SIZE: usize = 8 +  // tx_id
        1 +  // tx_type
        8 +  // from_user_id
        8 +  // to_user_id
        8 +  // amount (i64)
        8 +  // timestamp
        1 +  // status
        4; // desc_len

// Структура бинарного заголовка
struct BinHeader {
    magic: u32,
    record_size: u32,
}

// Структура бинарного тела записи
#[derive(Debug, PartialEq)]
pub(crate) struct BinRecord {
    pub(crate) tx_type: u8,
    pub(crate) status: u8,
    pub(crate) desc_len: u32,
    pub(crate) tx_id: u64,
    pub(crate) from_user_id: u64,
    pub(crate) to_user_id: u64,
    pub(crate) amount: u64,
    pub(crate) timestamp: u64,
    pub(crate) description: String,
}

impl TryFrom<&TransactionRecord> for BinRecord {
    type Error = TransToBinError;
    fn try_from(record: &TransactionRecord) -> Result<Self, Self::Error> {
        let tx_type = match record.tx_type {
            TxType::DEPOSIT => 0,
            TxType::TRANSFER => 1,
            TxType::WITHDRAWAL => 2,
        };

        let status = match record.status {
            Status::SUCCESS => 0,
            Status::FAILURE => 1,
            Status::PENDING => 2,
        };

        Ok(BinRecord {
            tx_type,
            status,
            desc_len: record.description.len() as u32,
            tx_id: record.tx_id,
            from_user_id: record.from_user_id,
            to_user_id: record.to_user_id,
            amount: record.amount,
            timestamp: record.timestamp,
            description: record.description.clone(),
        })
    }
}

/// Коллекция банковских записей, полученная из BIN-файла формата YP Bank.
///
/// Хранит вектор транзакций [`TransactionRecord`](crate::TransactionRecord).
/// Используется вместе с трейтом [`RecordParser`](crate::RecordParser) для чтения и записи.
#[derive(Debug, PartialEq)]
pub struct YPBankBinRecords {
    /// Вектор записей транзакций, извлечённых из BIN.
    pub records: Vec<TransactionRecord>,
}

impl YPBankBinRecords {
    pub fn new(records: Vec<TransactionRecord>) -> Self {
        YPBankBinRecords { records }
    }
}

impl RecordParser for YPBankBinRecords {
    fn from_read<R: Read>(r: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let mut reader = BufReader::new(r);
        let records = parse_bin_records(&mut reader)?;

        Ok(YPBankBinRecords { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> std::io::Result<()> {
        for record in self.records.iter() {
            let bin_record = BinRecord::try_from(record)?;
            write_record_to(writer, &bin_record)?;
        }

        Ok(())
    }
}

fn parse_bin_records<R: Read>(r: &mut R) -> std::io::Result<Vec<TransactionRecord>> {
    let mut records = Vec::new();
    loop {
        let header = match read_bin_header(r) {
            Ok(header) => header,
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        if header.magic != MAGIC {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid magic 0x{:X}", header.magic),
            ));
        }

        let mut buffer = vec![0u8; header.record_size as usize];
        r.read_exact(&mut buffer)?;

        let record = parse_record_from_bytes(buffer.as_slice())?;
        records.push(record);
    }

    Ok(records)
}

fn read_bin_header<R: Read>(r: &mut R) -> std::io::Result<BinHeader> {
    let magic = r.read_u32::<BigEndian>()?;
    let record_size = r.read_u32::<BigEndian>()?;

    Ok(BinHeader { magic, record_size })
}

fn parse_record_from_bytes(bytes: &[u8]) -> std::io::Result<TransactionRecord> {
    let mut cursor = Cursor::new(bytes);

    let tx_id = cursor.read_u64::<BigEndian>()?;
    let tx_type = match cursor.read_u8()? {
        0 => TxType::DEPOSIT,
        1 => TxType::TRANSFER,
        2 => TxType::WITHDRAWAL,
        other => return Err(BinToTransError::InvalidTxType(other).into()),
    };

    let from_user_id = cursor.read_u64::<BigEndian>()?;
    let to_user_id = cursor.read_u64::<BigEndian>()?;

    let amount = cursor.read_u64::<BigEndian>()?;
    let timestamp = cursor.read_u64::<BigEndian>()?;

    let status = match cursor.read_u8()? {
        0 => Status::SUCCESS,
        1 => Status::FAILURE,
        2 => Status::PENDING,
        other => return Err(Error::from(BinToTransError::InvalidStatus(other))),
    };

    let desc_len = cursor.read_u32::<BigEndian>()?;

    // Проверяем, что осталось достаточно байт для описания
    let remaining_bytes = bytes.len() - cursor.position() as usize;
    if desc_len as usize > remaining_bytes {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Not enough bytes for description: need {}, have {}",
                desc_len, remaining_bytes
            ),
        ));
    }

    let mut description_bytes = vec![0u8; desc_len as usize];
    cursor.read_exact(&mut description_bytes)?;

    let description =
        String::from_utf8(description_bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    let description = description.trim_matches('"').to_string();

    Ok(TransactionRecord {
        tx_type,
        status,
        tx_id,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        description,
    })
}

fn write_record_to<W: Write>(w: &mut W, record: &BinRecord) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(w);

    let body_size = BODY_FIXED_PART_SIZE + record.desc_len as usize;

    buffer.write_all(&MAGIC.to_be_bytes())?;
    buffer.write_all(&(body_size as u32).to_be_bytes())?;

    buffer.write_all(&record.tx_id.to_be_bytes())?;
    buffer.write_all(&[record.tx_type])?;
    buffer.write_all(&record.from_user_id.to_be_bytes())?;
    buffer.write_all(&record.to_user_id.to_be_bytes())?;
    buffer.write_all(&record.amount.to_be_bytes())?;
    buffer.write_all(&record.timestamp.to_be_bytes())?;
    buffer.write_all(&[record.status])?;
    buffer.write_all(&record.desc_len.to_be_bytes())?;
    buffer.write_all(record.description.as_bytes())?;
    buffer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_write_bin_records() {
        let mut test_bin_records = YPBankBinRecords {
            records: vec![TransactionRecord {
                tx_type: TxType::DEPOSIT,
                status: Status::FAILURE,
                tx_id: 1000000000000999,
                from_user_id: 0,
                to_user_id: 3314635390654657431,
                amount: 100000,
                timestamp: 1633096800000,
                description: "Record number 1000".to_string(),
            }],
        };

        let mut buffer = Cursor::new(Vec::new());
        test_bin_records.write_to(&mut buffer).unwrap();
        buffer.set_position(0);

        let buff_record = YPBankBinRecords::from_read(&mut buffer).unwrap();
        assert_eq!(test_bin_records, buff_record);
    }

    #[test]
    fn test_invalid_magic() {
        let mut data = Vec::new();
        data.extend_from_slice(&0xDEADBEEFu32.to_be_bytes()); // wrong magic
        data.extend_from_slice(&(100u32).to_be_bytes()); // record_size (не имеет значения)

        let mut cursor = Cursor::new(data);
        let result = YPBankBinRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        let msg = err.to_string();
        assert!(msg.contains("Invalid magic 0xDEADBEEF"));
    }

    #[test]
    fn test_invalid_tx_type() {
        // Формируем корректный заголовок и тело с недопустимым tx_type (3)
        let mut data = Vec::new();
        // Заголовок
        data.extend_from_slice(&MAGIC.to_be_bytes());
        let body_size = BODY_FIXED_PART_SIZE + 5; // desc_len = 5
        data.extend_from_slice(&(body_size as u32).to_be_bytes());

        // Тело
        data.extend_from_slice(&1u64.to_be_bytes());
        data.push(3); // tx_type -> invalid
        data.extend_from_slice(&2u64.to_be_bytes());
        data.extend_from_slice(&3u64.to_be_bytes());
        data.extend_from_slice(&100u64.to_be_bytes());
        data.extend_from_slice(&123456u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&5u32.to_be_bytes());
        data.extend_from_slice(b"hello");

        let mut cursor = Cursor::new(data);
        let result = YPBankBinRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(
            err.to_string()
                .contains("Invalid transaction type value: 3")
        );
    }

    #[test]
    fn test_invalid_status() {
        let mut data = Vec::new();
        data.extend_from_slice(&MAGIC.to_be_bytes());
        let body_size = BODY_FIXED_PART_SIZE + 5;
        data.extend_from_slice(&(body_size as u32).to_be_bytes());

        data.extend_from_slice(&1u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&2u64.to_be_bytes());
        data.extend_from_slice(&3u64.to_be_bytes());
        data.extend_from_slice(&100u64.to_be_bytes());
        data.extend_from_slice(&123456u64.to_be_bytes());
        data.push(5); // status -> invalid (5)
        data.extend_from_slice(&5u32.to_be_bytes());
        data.extend_from_slice(b"hello");

        let mut cursor = Cursor::new(data);
        let result = YPBankBinRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(
            err.to_string()
                .contains("Invalid transaction status value: 5")
        );
    }

    #[test]
    fn test_not_enough_bytes_for_description() {
        let mut data = Vec::new();
        data.extend_from_slice(&MAGIC.to_be_bytes());
        let body_size = BODY_FIXED_PART_SIZE + 5; // заявляем 10 байт под описание
        data.extend_from_slice(&(body_size as u32).to_be_bytes());

        data.extend_from_slice(&1u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&2u64.to_be_bytes());
        data.extend_from_slice(&3u64.to_be_bytes());
        data.extend_from_slice(&100u64.to_be_bytes());
        data.extend_from_slice(&123456u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&10u32.to_be_bytes()); // desc_len = 10
        // description занимает только 5 байт, но по заголовку должны быть 10
        data.extend_from_slice(b"hello");

        let mut cursor = Cursor::new(data);
        let result = YPBankBinRecords::from_read(&mut cursor);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(
            err.to_string()
                .contains("Not enough bytes for description: need 10, have 5")
        );
    }
}
