//! [ExtraData](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/c41e062d-f764-4f13-bd4f-ea812ab9a4d1) related structs

mod tracker_data_block;

use std::io::{Result, Seek, Read, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use tracker_data_block::TrackerDataBlock;

/// ExtraData types implemented
#[derive(Debug, Serialize)]
pub enum ExtraDataTypes {
    Tracker(TrackerDataBlock)
}

/// ExtraData refers to a set of structures that convey additional information about a link target. 
/// These optional structures can be present in an extra data section that is appended to the basic Shell Link Binary File Format.
#[derive(Debug, Serialize)]
pub struct ExtraData {
    pub extra_data_blocks: Vec<ExtraDataTypes>
}

impl ExtraData {
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read+Seek>(r: &mut R) -> Result<Self>{
        let mut extra_data_blocks: Vec<ExtraDataTypes> = Vec::new();
        loop {
            let size = r.read_u32::<LittleEndian>()?;
            if size == 0 {
                break;
            }
            let signature = r.read_u32::<LittleEndian>()?;
            let mut extra_data_bytes = vec![0;(size - 8) as usize];
            r.read_exact(&mut extra_data_bytes)?;
            match signature {
                0xa0000003 => { extra_data_blocks.push(ExtraDataTypes::Tracker(TrackerDataBlock::from_buffer(&extra_data_bytes)?)); },
                _ => {}
            };
        }

        Ok(Self {
            extra_data_blocks
        })
    }
}