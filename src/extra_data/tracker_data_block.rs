use winparsingtools::{structs::Guid, utils::read_cp1252_string, ReaderError};
use std::io::{Read, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TrackerDataBlock {
    #[serde(skip_serializing)]
    pub size: u32,
    #[serde(skip_serializing)]
    pub version: u32,
    pub machine_id: String,
    pub file_droid: Guid,
    pub file_droid_birth: Guid,
    pub volume_droid: Guid,
    pub volume_droid_birth: Guid

}

impl TrackerDataBlock {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, ReaderError>{
        let size = r.read_u32::<LittleEndian>()?;
        let version = r.read_u32::<LittleEndian>()?;
        let mut machine_id_bytes = [0;16];
        r.read_exact(&mut machine_id_bytes)?;
        let machine_id = read_cp1252_string(&mut Cursor::new(machine_id_bytes), None)?;
        let volume_droid = Guid::from_reader(r)?;
        let file_droid = Guid::from_reader(r)?;
        let volume_droid_birth = Guid::from_reader(r)?;
        let file_droid_birth = Guid::from_reader(r)?;

        Ok(Self {
            size,
            version,
            machine_id,
            file_droid,
            volume_droid,
            file_droid_birth,
            volume_droid_birth
        })
    }
}