mod bin_format;
mod csv_format;
mod error;
mod txt_format;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::io::{Read, Result, Write};

use strum::EnumString;

use crate::error::ConversionError;

use crate::bin_format::BinRecord;
pub use bin_format::YPBankBinRecords;
pub use csv_format::YPBankCsvRecords;
pub use txt_format::YPBankTxtRecords;

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

impl TryFrom<BinRecord> for TransactionRecord {
    type Error = ConversionError;

    fn try_from(record: BinRecord) -> std::result::Result<Self, Self::Error> {
        let tx_type = match record.tx_type {
            0 => TxType::DEPOSIT,
            1 => TxType::TRANSFER,
            2 => TxType::WITHDRAWAL,
            other => return Err(ConversionError::InvalidTxType(other)),
        };

        let status = match record.status {
            0 => Status::SUCCESS,
            1 => Status::FAILURE,
            2 => Status::PENDING,
            other => return Err(ConversionError::InvalidStatus(other)),
        };

        if (record.description.len() as u32) != record.desc_len {
            return Err(ConversionError::DescriptionLengthMismatch {
                expected: record.desc_len,
                actual: record.description.len(),
            });
        }

        Ok(TransactionRecord {
            tx_id: record.tx_id,
            tx_type,
            from_user_id: record.from_user_id,
            to_user_id: record.to_user_id,
            amount: record.amount,
            timestamp: record.timestamp,
            status,
            description: record.description,
        })
    }
}

pub trait RecordParser {
    fn from_read<R: Read>(_r: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write_to<W: Write>(&mut self, _writer: &mut W) -> Result<()> {
        todo!()
    }
}
