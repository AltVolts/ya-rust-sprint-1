use std::io::{BufRead, BufReader, Read};
mod bin_format;
mod csv_format;
mod txt_format;
mod error;

const MAGIC: u32 = 0x5950424E;
#[derive(Debug)]
pub struct YPBankBinRecord {
    marker: u32,
    body: Vec<u8>,
}

impl YPBankBinRecord {
    pub fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self, std::io::Error> {

        todo!()
    }

    pub fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), std::io::Error> {
        todo!()
    }
}
