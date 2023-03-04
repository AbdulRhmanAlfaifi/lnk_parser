//! [LinkInfo](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/6813269d-0cc8-4be2-933f-e96e8e3412dc) related structs

mod common_network_relative_link;
mod volume_id;
use byteorder::{LittleEndian, ReadBytesExt};
pub use common_network_relative_link::CommonNetworkRelativeLink;
use getset::Getters;
use serde::{Serialize, Serializer};
use winparsingtools::ReaderError;
use std::fmt::{self, Display};
use std::io::{Cursor, Read, Seek, SeekFrom};
pub use volume_id::VolumeID;
use winparsingtools::{traits::Path, utils};

/// The LinkInfo structure specifies information necessary to resolve a link target if it is not found in its original location.
#[derive(Debug, Serialize, Getters)]
#[getset(get = "pub with_prefix")]
pub struct LinkInfo {
    #[serde(skip_serializing)]
    pub size: u32,
    #[serde(skip_serializing)]
    pub header_size: u32,
    #[serde(skip_serializing)]
    pub flags: LinkInfoFlags,
    #[serde(skip_serializing)]
    pub volume_id_offset: u32,
    #[serde(skip_serializing)]
    pub local_base_path_offset: u32,
    #[serde(skip_serializing)]
    pub common_network_relative_link_offset: u32,
    #[serde(skip_serializing)]
    pub common_path_suffix_offset: u32,
    #[serde(skip_serializing)]
    pub local_base_path_offset_unicode: Option<u32>,
    #[serde(skip_serializing)]
    pub common_path_suffix_offset_unicode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<VolumeID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_base_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_network_relative_link: Option<CommonNetworkRelativeLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_path_suffix: Option<String>,
}

impl LinkInfo {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        let size = r.read_u32::<LittleEndian>()?;
        let mut link_info_data = vec![0; (size - 4) as usize];
        r.read_exact(&mut link_info_data)?;
        let r = &mut Cursor::new(link_info_data);
        let header_size = r.read_u32::<LittleEndian>()?;
        let flags = LinkInfoFlags::from_u32(r.read_u32::<LittleEndian>()?)?;
        let volume_id_offset = r.read_u32::<LittleEndian>()?;
        let local_base_path_offset = r.read_u32::<LittleEndian>()?;
        let common_network_relative_link_offset = r.read_u32::<LittleEndian>()?;
        let common_path_suffix_offset = r.read_u32::<LittleEndian>()?;

        let mut local_base_path_offset_unicode = None;
        let mut common_path_suffix_offset_unicode = None;

        // Only available if the header size is greater than or equal to 0x24
        if header_size >= 0x24 {
            local_base_path_offset_unicode = Some(r.read_u32::<LittleEndian>()?);
            common_path_suffix_offset_unicode = Some(r.read_u32::<LittleEndian>()?);
        }

        let mut volume_id = None;
        let mut common_network_relative_link = None;

        if flags.VolumeIDAndLocalBasePath {
            r.seek(SeekFrom::Start((volume_id_offset - 4) as u64))?;
            volume_id = Some(VolumeID::from_reader(r)?);
        }

        if flags.CommonNetworkRelativeLinkAndPathSuffix {
            r.seek(SeekFrom::Start(
                (common_network_relative_link_offset - 4) as u64,
            ))?;
            common_network_relative_link = Some(CommonNetworkRelativeLink::from_reader(r)?);
        }

        // Read unicode local_base_path if available, else read normal local_base_path
        let local_base_path = match local_base_path_offset_unicode {
            Some(offset) => match offset {
                0 => None,
                _ => {
                    r.seek(SeekFrom::Start((offset - 4) as u64))?;
                    match utils::read_utf16_string(r, None)? {
                        s if !s.is_empty() => Some(s),
                        _ => None,
                    }
                }
            },
            None => match local_base_path_offset {
                0 => None,
                offset => {
                    r.seek(SeekFrom::Start((offset - 4) as u64))?;
                    match utils::read_cp1252_string(r, None)? {
                        s if !s.is_empty() => Some(s),
                        _ => None,
                    }
                }
            },
        };

        let common_path_suffix = match common_path_suffix_offset_unicode {
            Some(offset) => match offset {
                0 => None,
                _ => {
                    r.seek(SeekFrom::Start((offset - 4) as u64))?;
                    match utils::read_utf16_string(r, None)? {
                        s if !s.is_empty() => Some(s),
                        _ => None,
                    }
                }
            },
            None => match common_path_suffix_offset {
                0 => None,
                offset => {
                    r.seek(SeekFrom::Start((offset - 4) as u64))?;
                    match utils::read_cp1252_string(r, None)? {
                        s if !s.is_empty() => Some(s),
                        _ => None,
                    }
                }
            },
        };

        Ok(LinkInfo {
            size,
            header_size,
            flags,
            volume_id_offset,
            local_base_path_offset,
            common_network_relative_link_offset,
            common_path_suffix_offset,
            local_base_path_offset_unicode,
            common_path_suffix_offset_unicode,
            volume_id,
            local_base_path,
            common_network_relative_link,
            common_path_suffix,
        })
    }
}

impl Path for LinkInfo {
    fn path(&self) -> Option<String> {
        let path = match &self.local_base_path {
            Some(local_base_path) => match &self.common_path_suffix {
                // if `common_path_suffix` and `local_base_path` are present then return {local_base_path}\{common_path_suffix}
                Some(common_path_suffix) => Some(
                    format!("{}\\{}", local_base_path, common_path_suffix).replace("\\\\", "\\"),
                ),
                None => Some(local_base_path.to_string().replace("\\\\", "\\")),
            },
            None => None,
        };

        match path {
            Some(p) => Some(p),
            None => match &self.common_network_relative_link {
                Some(common_network_relative_link) => common_network_relative_link.path(),
                None => None,
            },
        }
    }
}

/// Flags that specify whether the VolumeID, LocalBasePath, LocalBasePathUnicode, and CommonNetworkRelativeLink fields are present in this structure.
#[derive(Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct LinkInfoFlags {
    VolumeIDAndLocalBasePath: bool,
    CommonNetworkRelativeLinkAndPathSuffix: bool,
}

impl LinkInfoFlags {
    pub fn new(
        VolumeIDAndLocalBasePath: bool,
        CommonNetworkRelativeLinkAndPathSuffix: bool,
    ) -> Result<LinkInfoFlags, ReaderError> {
        Ok(LinkInfoFlags {
            VolumeIDAndLocalBasePath,
            CommonNetworkRelativeLinkAndPathSuffix,
        })
    }

    pub fn from_u32(flags: u32) -> Result<LinkInfoFlags, ReaderError> {
        Ok(LinkInfoFlags {
            VolumeIDAndLocalBasePath: (flags & 0x01 != 0),
            CommonNetworkRelativeLinkAndPathSuffix: (flags & 0x02 != 0),
        })
    }
}

impl Display for LinkInfoFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = vec![];
        if self.VolumeIDAndLocalBasePath {
            result.push("VolumeIDAndLocalBasePath");
        }
        if self.CommonNetworkRelativeLinkAndPathSuffix {
            result.push("CommonNetworkRelativeLinkAndPathSuffix");
        }
        write!(f, "{}", result.join(","))
    }
}

impl Serialize for LinkInfoFlags {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self.to_string().split(',').collect::<Vec<&str>>())
    }
}
