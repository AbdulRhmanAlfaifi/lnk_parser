//! [CommonNetworkRelativeLink](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/23bb5877-e3dd-4799-9f50-79f05f938537) related structs 

use getset::Getters;
use winparsingtools::{
    utils,
    traits::Path
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Result, Seek, SeekFrom};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum CommonNetworkRelativeLinkFlags {
    ValidDevice,
    ValidNetType,
}
#[derive(Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum NetworkProviderType {
    WNNC_NET_MSNET,
    WNNC_NET_SMB_LANMAN,
    WNNC_NET_NETWARE,
    WNNC_NET_VINES,
    WNNC_NET_10NET,
    WNNC_NET_LOCUS,
    WNNC_NET_SUN_PC_NFS,
    WNNC_NET_LANSTEP,
    WNNC_NET_9TILES,
    WNNC_NET_LANTASTIC,
    WNNC_NET_AS400,
    WNNC_NET_FTP_NFS,
    WNNC_NET_PATHWORKS,
    WNNC_NET_LIFENET,
    WNNC_NET_POWERLAN,
    WNNC_NET_BWNFS,
    WNNC_NET_COGENT,
    WNNC_NET_FARALLON,
    WNNC_NET_APPLETALK,
    WNNC_NET_INTERGRAPH,
    WNNC_NET_SYMFONET,
    WNNC_NET_CLEARCASE,
    WNNC_NET_FRONTIER,
    WNNC_NET_BMC,
    WNNC_NET_DCE,
    WNNC_NET_AVID,
    WNNC_NET_DOCUSPACE,
    WNNC_NET_MANGOSOFT,
    WNNC_NET_SERNET,
    WNNC_NET_RIVERFRONT1,
    WNNC_NET_RIVERFRONT2,
    WNNC_NET_DECORB,
    WNNC_NET_PROTSTOR,
    WNNC_NET_FJ_REDIR,
    WNNC_NET_DISTINCT,
    WNNC_NET_TWINS,
    WNNC_NET_RDR2SAMPLE,
    WNNC_NET_CSC,
    WNNC_NET_3IN1,
    WNNC_NET_EXTENDNET,
    WNNC_NET_STAC,
    WNNC_NET_FOXBAT,
    WNNC_NET_YAHOO,
    WNNC_NET_EXIFS,
    WNNC_NET_DAV,
    WNNC_NET_KNOWARE,
    WNNC_NET_OBJECT_DIRE,
    WNNC_NET_MASFAX,
    WNNC_NET_HOB_NFS,
    WNNC_NET_SHIVA,
    WNNC_NET_IBMAL,
    WNNC_NET_LOCK,
    WNNC_NET_TERMSRV,
    WNNC_NET_SRT,
    WNNC_NET_QUINCY,
    WNNC_NET_OPENAFS,
    WNNC_NET_AVID1,
    WNNC_NET_DFS,
    WNNC_NET_KWNP,
    WNNC_NET_ZENWORKS,
    WNNC_NET_DRIVEONWEB,
    WNNC_NET_VMWARE,
    WNNC_NET_RSFX,
    WNNC_NET_MFILES,
    WNNC_NET_MS_NFS,
    WNNC_NET_GOOGLE,
    WNNC_NET_NDFS,
    WNNC_NET_DOCUSHARE,
    WNNC_CRED_MANAGER,
    UNKNOWN,
}

impl From<u32> for NetworkProviderType {
    fn from(num: u32) -> Self {
        match num {
            0x00010000 => NetworkProviderType::WNNC_NET_MSNET,
            0x00020000 => NetworkProviderType::WNNC_NET_SMB_LANMAN,
            0x00030000 => NetworkProviderType::WNNC_NET_NETWARE,
            0x00040000 => NetworkProviderType::WNNC_NET_VINES,
            0x00050000 => NetworkProviderType::WNNC_NET_10NET,
            0x00060000 => NetworkProviderType::WNNC_NET_LOCUS,
            0x00070000 => NetworkProviderType::WNNC_NET_SUN_PC_NFS,
            0x00080000 => NetworkProviderType::WNNC_NET_LANSTEP,
            0x00090000 => NetworkProviderType::WNNC_NET_9TILES,
            0x000A0000 => NetworkProviderType::WNNC_NET_LANTASTIC,
            0x000B0000 => NetworkProviderType::WNNC_NET_AS400,
            0x000C0000 => NetworkProviderType::WNNC_NET_FTP_NFS,
            0x000D0000 => NetworkProviderType::WNNC_NET_PATHWORKS,
            0x000E0000 => NetworkProviderType::WNNC_NET_LIFENET,
            0x000F0000 => NetworkProviderType::WNNC_NET_POWERLAN,
            0x00100000 => NetworkProviderType::WNNC_NET_BWNFS,
            0x00110000 => NetworkProviderType::WNNC_NET_COGENT,
            0x00120000 => NetworkProviderType::WNNC_NET_FARALLON,
            0x00130000 => NetworkProviderType::WNNC_NET_APPLETALK,
            0x00140000 => NetworkProviderType::WNNC_NET_INTERGRAPH,
            0x00150000 => NetworkProviderType::WNNC_NET_SYMFONET,
            0x00160000 => NetworkProviderType::WNNC_NET_CLEARCASE,
            0x00170000 => NetworkProviderType::WNNC_NET_FRONTIER,
            0x00180000 => NetworkProviderType::WNNC_NET_BMC,
            0x00190000 => NetworkProviderType::WNNC_NET_DCE,
            0x001A0000 => NetworkProviderType::WNNC_NET_AVID,
            0x001B0000 => NetworkProviderType::WNNC_NET_DOCUSPACE,
            0x001C0000 => NetworkProviderType::WNNC_NET_MANGOSOFT,
            0x001D0000 => NetworkProviderType::WNNC_NET_SERNET,
            0x001E0000 => NetworkProviderType::WNNC_NET_RIVERFRONT1,
            0x001F0000 => NetworkProviderType::WNNC_NET_RIVERFRONT2,
            0x00200000 => NetworkProviderType::WNNC_NET_DECORB,
            0x00210000 => NetworkProviderType::WNNC_NET_PROTSTOR,
            0x00220000 => NetworkProviderType::WNNC_NET_FJ_REDIR,
            0x00230000 => NetworkProviderType::WNNC_NET_DISTINCT,
            0x00240000 => NetworkProviderType::WNNC_NET_TWINS,
            0x00250000 => NetworkProviderType::WNNC_NET_RDR2SAMPLE,
            0x00260000 => NetworkProviderType::WNNC_NET_CSC,
            0x00270000 => NetworkProviderType::WNNC_NET_3IN1,
            0x00290000 => NetworkProviderType::WNNC_NET_EXTENDNET,
            0x002A0000 => NetworkProviderType::WNNC_NET_STAC,
            0x002B0000 => NetworkProviderType::WNNC_NET_FOXBAT,
            0x002C0000 => NetworkProviderType::WNNC_NET_YAHOO,
            0x002D0000 => NetworkProviderType::WNNC_NET_EXIFS,
            0x002E0000 => NetworkProviderType::WNNC_NET_DAV,
            0x002F0000 => NetworkProviderType::WNNC_NET_KNOWARE,
            0x00300000 => NetworkProviderType::WNNC_NET_OBJECT_DIRE,
            0x00310000 => NetworkProviderType::WNNC_NET_MASFAX,
            0x00320000 => NetworkProviderType::WNNC_NET_HOB_NFS,
            0x00330000 => NetworkProviderType::WNNC_NET_SHIVA,
            0x00340000 => NetworkProviderType::WNNC_NET_IBMAL,
            0x00350000 => NetworkProviderType::WNNC_NET_LOCK,
            0x00360000 => NetworkProviderType::WNNC_NET_TERMSRV,
            0x00370000 => NetworkProviderType::WNNC_NET_SRT,
            0x00380000 => NetworkProviderType::WNNC_NET_QUINCY,
            0x00390000 => NetworkProviderType::WNNC_NET_OPENAFS,
            0x003A0000 => NetworkProviderType::WNNC_NET_AVID1,
            0x003B0000 => NetworkProviderType::WNNC_NET_DFS,
            0x003C0000 => NetworkProviderType::WNNC_NET_KWNP,
            0x003D0000 => NetworkProviderType::WNNC_NET_ZENWORKS,
            0x003E0000 => NetworkProviderType::WNNC_NET_DRIVEONWEB,
            0x003F0000 => NetworkProviderType::WNNC_NET_VMWARE,
            0x00400000 => NetworkProviderType::WNNC_NET_RSFX,
            0x00410000 => NetworkProviderType::WNNC_NET_MFILES,
            0x00420000 => NetworkProviderType::WNNC_NET_MS_NFS,
            0x00430000 => NetworkProviderType::WNNC_NET_GOOGLE,
            0x00440000 => NetworkProviderType::WNNC_NET_NDFS,
            0x00450000 => NetworkProviderType::WNNC_NET_DOCUSHARE,
            0xFFFF0000 => NetworkProviderType::WNNC_CRED_MANAGER,
            _ => NetworkProviderType::UNKNOWN,
        }
    }
}
/// The CommonNetworkRelativeLink structure specifies information about the network location where a
/// link target is stored, including the mapped drive letter and the UNC path prefix.
#[derive(Debug, Serialize, Getters)]
#[getset(get = "pub with_prefix")]
pub struct CommonNetworkRelativeLink {
    #[serde(skip_serializing)]
    pub size: u32,
    pub flags: Vec<CommonNetworkRelativeLinkFlags>,
    #[serde(skip_serializing)]
    pub net_name_offset: u32,
    #[serde(skip_serializing)]
    pub device_name_offset: u32,
    pub network_provider_type: Option<NetworkProviderType>,
    #[serde(skip_serializing)]
    pub net_name_offset_unicode: Option<u32>,
    #[serde(skip_serializing)]
    pub device_name_offset_unicode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>
}

impl CommonNetworkRelativeLink {
    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let size = r.read_u32::<LittleEndian>()?;
        let mut common_network_relative_link_data = vec![0;(size - 4) as usize];
        r.read_exact(&mut common_network_relative_link_data)?;
        let r = & mut Cursor::new(common_network_relative_link_data);
        let mut flags: Vec<CommonNetworkRelativeLinkFlags> = vec![];
        match r.read_u32::<LittleEndian>()? {
            num if num & 1 > 0 => flags.push(CommonNetworkRelativeLinkFlags::ValidDevice),
            num if num & 2 > 0 => flags.push(CommonNetworkRelativeLinkFlags::ValidNetType),
            _ => {}
        };
        let net_name_offset = r.read_u32::<LittleEndian>()?;
        let device_name_offset = r.read_u32::<LittleEndian>()?;
        let mut network_provider_type = None;
        let mut net_name_offset_unicode = None;
        let mut device_name_offset_unicode = None;
        let mut device_name = None;
        if flags.iter().any(|f| matches!(f, CommonNetworkRelativeLinkFlags::ValidNetType)) {

            network_provider_type = Some(NetworkProviderType::from(
                r.read_u32::<LittleEndian>()?
            ));
        }

        if net_name_offset > 0x14 {
            net_name_offset_unicode = Some(r.read_u32::<LittleEndian>()?);
        }

        if device_name_offset > 0x14 {
            device_name_offset_unicode = Some(r.read_u32::<LittleEndian>()?);
        }

        let net_name = match net_name_offset_unicode {
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
            None => match net_name_offset {
                0 => None,
                _ => {
                    r.seek(SeekFrom::Start((net_name_offset-4) as u64))?;
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


        // if ValidDevice flag is set then read the device name.
        if flags.iter().any(|f| matches!(f, CommonNetworkRelativeLinkFlags::ValidDevice)) {
            device_name = match device_name_offset_unicode {
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
                None => match device_name_offset {
                    0 => None,
                    _ => {
                        r.seek(SeekFrom::Start((device_name_offset-4) as u64))?;
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
        }


        Ok(Self {
            size,
            flags,
            net_name_offset,
            device_name_offset,
            network_provider_type,
            net_name_offset_unicode,
            device_name_offset_unicode,
            net_name,
            device_name
        })
    }
}

impl Path for CommonNetworkRelativeLink {
    fn path(&self) -> Option<String> {
        match &self.net_name {
            Some(net_name) => match &self.device_name {
                Some(device_name) => Some(format!("{}\\{}", net_name, device_name)),
                None => Some(net_name.to_owned())
            },
            None => None
        }
    }
}
