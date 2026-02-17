use crate::{Status, TxType};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum BinToTransError {
    #[error("Invalid transaction type value: {0}")]
    InvalidTxType(u8),
    #[error("Invalid status value: {0}")]
    InvalidStatus(u8),
    #[error("Description length mismatch: expected {expected}, actual {actual}")]
    DescriptionLengthMismatch { expected: u32, actual: usize },
}

impl From<BinToTransError> for std::io::Error {
    fn from(e: BinToTransError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}

#[derive(Error, Debug)]
pub(crate) enum TransToBinError {
    #[allow(dead_code)]
    #[error("Invalid transaction type: {0:?}")]
    UnsupportedTxType(TxType),
    #[allow(dead_code)]
    #[error("Invalid status value: {0:?}")]
    UnsupportedStatus(Status),
}

impl From<TransToBinError> for std::io::Error {
    fn from(e: TransToBinError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}
