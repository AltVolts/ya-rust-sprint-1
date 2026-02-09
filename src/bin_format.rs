use std::io::{Read, Write, Result, BufReader};

use crate::{RecordParser};

const MAGIC: u32 = 0x5950424E;

struct BinRecord {
    tx_type: u8,
    status: u8,
    desc_len: u32,
    tx_id: u64,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    description: Vec<u8>,
}

#[derive(Debug)]
pub struct YPBankBinRecord {
    marker: u32,
    // body: Vec<u8>,
}

fn get_32_be<R: Read>(r: &mut R) -> Result<u32> {
    let mut bytes= [0u8; 4];
    r.read_exact(&mut bytes)?;

    println!("{:?}", bytes);
    Ok(u32::from_be_bytes(bytes))
}

impl RecordParser for YPBankBinRecord {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut reader = BufReader::new(r);
        let magic = get_32_be(&mut reader)?;
        if magic != MAGIC {
            println!("Wrong magic number");
        }
        Ok(YPBankBinRecord{marker: magic})

        // while let Ok(record) = read_record(&mut reader) {
        //     records.push(record);
        // }
        // reader.bytes().take()
        //
        // println!("{buffer}");
        // Ok(YPBankBinRecord{})
    }

    fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<()> {
        todo!()
    }
}
