use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    InvalidTxType(u8),
    InvalidStatus(u8),
    DescriptionLengthMismatch { expected: u32, actual: usize },
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::InvalidTxType(val) => {
                write!(f, "Invalid transaction type value: {}", val)
            }
            ConversionError::InvalidStatus(val) => write!(f, "Invalid status value: {}", val),
            ConversionError::DescriptionLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "Description length mismatch: expected {}, actual {}",
                    expected, actual
                )
            }
        }
    }
}

impl std::error::Error for ConversionError {}
