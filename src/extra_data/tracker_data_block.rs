use byteorder::{LittleEndian, ReadBytesExt};
use serde::{ser::SerializeStruct, Serialize};
use std::io::{Cursor, Read};
use winparsingtools::{structs::Guid, utils::read_cp1252_string, ReaderError};

#[derive(Debug)]
pub struct TrackerDataBlock {
    pub size: u32,
    pub version: u32,
    pub machine_id: String,
    pub file_droid: Guid,
    pub file_droid_birth: Guid,
    pub volume_droid: Guid,
    pub volume_droid_birth: Guid,
}

impl TrackerDataBlock {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, ReaderError> {
        let size = r.read_u32::<LittleEndian>()?;
        let version = r.read_u32::<LittleEndian>()?;
        let mut machine_id_bytes = [0; 16];
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
            volume_droid_birth,
        })
    }

    pub fn get_mac_address(&self) -> String {
        // Ensure the UUID is valid and in the correct format
        let uuid = self.file_droid.to_string();

        // Check if the UUID is Version 1 (13th character must be '1')
        if uuid.len() == 36 && uuid.chars().nth(14) == Some('1') {
            // Extract the node portion (last 12 characters of the UUID)
            if let Some(node_part) = uuid.get(24..36) {
                // Split into MAC address format (every 2 characters separated by ':')
                return node_part
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<_>>()
                    .join(":")
                    .to_uppercase();
            }
        }

        // Return default MAC address if not Version 1 or invalid format
        "00:00:00:00:00:00".to_string()
    }
}

impl Serialize for TrackerDataBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TrackerDataBlock", 5)?;

        state.serialize_field("machine_id", &self.machine_id)?;
        state.serialize_field("file_droid", &self.file_droid)?;
        state.serialize_field("volume_droid", &self.volume_droid)?;
        state.serialize_field("file_droid_birth", &self.file_droid_birth)?;
        state.serialize_field("volume_droid_birth", &self.volume_droid_birth)?;
        state.serialize_field("mac_address", &self.get_mac_address())?;

        state.end()
    }
}
