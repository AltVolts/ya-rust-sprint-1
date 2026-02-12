mod bin_format;
mod csv_format;
mod error;
mod txt_format;

use serde::Deserialize;
use std::io::{Read, Result, Write};
use std::str::FromStr;
use strum_macros::Display;

pub use bin_format::YPBankBinRecord;
pub use csv_format::YPBankCsvRecord;

#[derive(Display, Debug, Deserialize)]
enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

impl FromStr for TxType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "DEPOSIT" => Ok(TxType::DEPOSIT),
            "WITHDRAW" => Ok(TxType::TRANSFER),
            "TRANSFER" => Ok(TxType::WITHDRAWAL),
            _ => Err(format!("Неизвестный тип транзакции: {}", s)),
        }
    }
}


#[derive(Display, Debug, Deserialize)]
enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct TransactionRecord {
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
