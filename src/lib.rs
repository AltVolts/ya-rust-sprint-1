mod bin_format;
mod csv_format;
mod error;
mod txt_format;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::io::{Read, Result, Write};

use strum::EnumString;

pub use bin_format::YPBankBinRecord;
pub use csv_format::YPBankCsvRecord;
pub use txt_format::YPBankTxtRecord;

#[derive(Debug, Deserialize, EnumString, Display, PartialEq, Serialize)]
enum TxType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

#[derive(Debug, Deserialize, EnumString, Display, PartialEq, Serialize)]
enum Status {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Debug, Deserialize, Display, Serialize)]
#[display(
    "TransactionRecord {{
    tx_id: {tx_id},
    tx_type: {tx_type},
    from_user_id: {from_user_id},
    to_user_id: {to_user_id},
    amount: {amount},
    timestamp: {timestamp},
    status: {status},
    description: {description},
}}"
)]
#[serde(rename_all = "UPPERCASE")]
#[derive(PartialEq)]
pub struct TransactionRecord {
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
