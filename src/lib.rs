mod bin_format;
mod csv_format;
mod txt_format;
mod error;

use std::io::{ Read, Write, Result};
use strum_macros::Display;

pub use bin_format::YPBankBinRecord;


#[derive(Display, Debug)]
enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAW,
}

#[derive(Display, Debug)]
enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Debug)]
struct Record {
    tx_id: u64,
    tx_type: TxType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    status: Status,
    description: String,
}

pub trait RecordParser {
    fn from_read<R: Read>(r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        todo!()
    }
}


