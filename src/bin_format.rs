use std::io::{Read, Write, Result, BufReader, ErrorKind, Error, BufWriter, Cursor};
use byteorder::{BigEndian, ReadBytesExt};

use crate::{RecordParser};

//Постоянное значение 0x59 0x50 0x42 0x4E ('YPBN'), идентифицирующее заголовок записи.
const MAGIC: u32 = 0x5950424E;

const BODY_FIXED_PART_SIZE: usize =
        8 +  // tx_id
        1 +  // tx_type
        8 +  // from_user_id
        8 +  // to_user_id
        8 +  // amount (i64)
        8 +  // timestamp
        1 +  // status
        4;  // desc_len

struct RecordHeader {
    magic: u32,
    record_size: u32,
}

#[derive(Debug)]
struct BinRecord {
    tx_type: u8,
    status: u8,
    desc_len: u32,
    tx_id: u64,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    description: String,
}

fn parse_bin_records<R: Read>(r: &mut R) -> Result<Vec<BinRecord>> {
    let mut records = Vec::new();
    loop {
        let header = match read_record_header(r) {
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

fn read_record_header<R: Read>(r: &mut R) -> Result<RecordHeader> {
    let magic = r.read_u32::<BigEndian>()?;
    let record_size = r.read_u32::<BigEndian>()?;

    Ok(RecordHeader { magic, record_size })
}

fn parse_record_from_bytes(bytes: &[u8]) -> Result<BinRecord> {
    let mut cursor = Cursor::new(bytes);

    let tx_id = cursor.read_u64::<BigEndian>()?;
    let tx_type = cursor.read_u8()?;

    let from_user_id = cursor.read_u64::<BigEndian>()?;
    let to_user_id = cursor.read_u64::<BigEndian>()?;

    let amount = cursor.read_u64::<BigEndian>()?;
    let timestamp = cursor.read_u64::<BigEndian>()?;


    let status = cursor.read_u8()?;
    let desc_len = cursor.read_u32::<BigEndian>()?;

    // Проверяем, что осталось достаточно байт для описания
    let remaining_bytes = bytes.len() - cursor.position() as usize;
    if desc_len as usize > remaining_bytes {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Not enough bytes for description: need {}, have {}",
                    desc_len, remaining_bytes)
        ));
    }

    let mut description_bytes = vec![0u8; desc_len as usize];
    cursor.read_exact(&mut description_bytes)?;

    let description = String::from_utf8(description_bytes)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(BinRecord {
        tx_type,
        status,
        desc_len,
        tx_id,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        description,
    })
}

#[derive(Debug)]
pub struct YPBankBinRecord {
    records: Vec<BinRecord>,
}

impl RecordParser for YPBankBinRecord {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut reader = BufReader::new(r);
        let records  = parse_bin_records(&mut reader)?;
        println!("{:#?}", records );
        Ok(YPBankBinRecord { records })
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        for record in &self.records {
            write_record_to(writer, &record)?;
        }

        Ok(())
    }
}

fn write_record_to<W: Write>(w: &mut W, record: &BinRecord) -> Result<()> {
    let mut buffer = BufWriter::new(w);

    // Рассчитываем размер тела записи
    let body_size = BODY_FIXED_PART_SIZE + record.desc_len as usize;

    // Записываем заголовок
    buffer.write_all(&MAGIC.to_be_bytes())?;
    buffer.write_all(&(body_size as u32).to_be_bytes())?;

    // Записываем тело записи
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
    use std::io::{Cursor, Seek, SeekFrom};

    #[test]
    fn test_write_bin_records() {
        let description = "Record number 1000".to_string();
        let desc_len = description.len() as u32;
        let test_bin_records = vec![
            BinRecord {
                tx_type: 0,
                status: 1,
                desc_len,
                tx_id: 1000000000000999,
                from_user_id: 0,
                to_user_id: 3314635390654657431,
                amount: 100000,
                timestamp: 1633096800000,
                description: "Record number 1000".to_string(),
            }
        ];
        println!("Full record size {:#?}", 8 + BODY_FIXED_PART_SIZE + desc_len as usize);
        let mut buffer = Cursor::new(Vec::new());
        write_record_to(&mut buffer, &test_bin_records[0]).unwrap();
        buffer.seek(SeekFrom::Start(0)).unwrap();

        let result = parse_bin_records(&mut buffer).unwrap();
        assert_eq!(result.len(), test_bin_records.len());

        let parsed_record = &result[0];
        let expected_record = &test_bin_records[0];

        assert_eq!(parsed_record.tx_id, expected_record.tx_id);
        assert_eq!(parsed_record.tx_type, expected_record.tx_type);
        assert_eq!(parsed_record.from_user_id, expected_record.from_user_id);
        assert_eq!(parsed_record.to_user_id, expected_record.to_user_id);
        assert_eq!(parsed_record.amount, expected_record.amount);
        assert_eq!(parsed_record.timestamp, expected_record.timestamp);
        assert_eq!(parsed_record.status, expected_record.status);
        assert_eq!(parsed_record.desc_len, expected_record.desc_len as u32);
        assert_eq!(parsed_record.description, expected_record.description);


    }
}