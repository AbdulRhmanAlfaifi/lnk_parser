//! [VolumeID](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/b7b3eea7-dbff-4275-bd58-83ba3f12d87a) related structs
use getset::Getters;
use winparsingtools::utils;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Result, Seek, SeekFrom};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum VolumeIDDriveType {
    DRIVE_UNKNOWN,     //The drive type cannot be determined.
    DRIVE_NO_ROOT_DIR, //The root path is invalid; for example, there is no volume mounted at the path.
    DRIVE_REMOVABLE, //The drive has removable media, such as a floppy drive, thumb drive, or flash card reader.
    DRIVE_FIXED,     //The drive has fixed media, such as a hard drive or flash drive.
    DRIVE_REMOTE,    //The drive is a remote (network) drive.
    DRIVE_CDROM,     //The drive is a CD-ROM drive.
    DRIVE_RAMDISK,   //The drive is a RAM disk.
}

impl From<u32> for VolumeIDDriveType {
    fn from(num: u32) -> Self {
        match num {
            1 => VolumeIDDriveType::DRIVE_NO_ROOT_DIR,
            2 => VolumeIDDriveType::DRIVE_REMOVABLE,
            3 => VolumeIDDriveType::DRIVE_FIXED,
            4 => VolumeIDDriveType::DRIVE_REMOTE,
            5 => VolumeIDDriveType::DRIVE_CDROM,
            6 => VolumeIDDriveType::DRIVE_RAMDISK,
            _ => VolumeIDDriveType::DRIVE_UNKNOWN,
        }
    }
}

/// The VolumeID structure specifies information about the volume that a link target was on when the link was created.
#[derive(Debug, Serialize, Getters)]
#[getset(get = "pub with_prefix")]
pub struct VolumeID {
    #[serde(skip_serializing)]
    size: u32,
    drive_type: VolumeIDDriveType,
    serial_number: String,
    #[serde(skip_serializing)]
    volume_label_offset: u32,
    #[serde(skip_serializing)]
    volume_label_offset_unicode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_label: Option<String>,
}

impl VolumeID {
    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let size = r.read_u32::<LittleEndian>()?;
        let mut volume_id_data = vec![0;(size - 4) as usize];
        r.read_exact(&mut volume_id_data)?;
        let r = & mut Cursor::new(volume_id_data);
        let drive_type = VolumeIDDriveType::from(r.read_u32::<LittleEndian>()?);
        let serial = r.read_u32::<LittleEndian>()?;
        // format the serial number as XXXX-XXXX
        let serial_number = format!("{:X}-{:X}",serial >> 16, serial & 0x0000ffff);
        let volume_label_offset = r.read_u32::<LittleEndian>()?;
        let mut volume_label_offset_unicode = None;

        if volume_label_offset == 0x14 {
            // it is a unicode string
            volume_label_offset_unicode = Some(r.read_u32::<LittleEndian>()?);
        } 

        let volume_label = match volume_label_offset_unicode {
            Some(offset) => match offset {
                0 => None,
                _ => {
                        r.seek(SeekFrom::Start((offset-4) as u64))?;
                        match utils::read_utf16_string(r, None) {
                        Ok(s) => match s {
                            s if !s.is_empty() => Some(s),
                            _ => None
                        },
                        Err(_) => None,
                    }
                }
            },
            None => match volume_label_offset {
                0 => None,
                _ => {
                    r.seek(SeekFrom::Start((volume_label_offset-4) as u64))?;
                    match utils::read_cp1252_string(r, None) {
                    Ok(s) => match s {
                        s if !s.is_empty() => Some(s),
                        _ => None
                    },
                    Err(_) => None,
                }
            }
            }
        };

        Ok(Self {
            size,
            drive_type,
            serial_number,
            volume_label_offset,
            volume_label_offset_unicode,
            volume_label,
        })
    }
}