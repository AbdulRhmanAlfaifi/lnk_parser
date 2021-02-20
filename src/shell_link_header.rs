//! [ShellLinkHeader](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/c3376b21-0931-45e4-b2fc-a48ac0e60d15) related structs

use winparsingtools::structs::Guid;
use winparsingtools::date_time::FileTime;
use winparsingtools::file_system::FileAttributesFlags;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Result};
use serde::ser;
use std::fmt::{self,Display};
use serde::Serialize;

/* #region  LinkFlags Struct Implementation */

/// The [LinkFlags](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/ae350202-3ba9-4790-9e9e-98935f4ee5af) structure defines bits that specify which shell link structures are present in the file format after the ShellLinkHeader structure
#[derive(Debug)]
pub struct LinkFlags{
    pub HasLinkTargetIDList: bool,
    pub HasLinkInfo: bool,
    pub HasName: bool,
    pub HasRelativePath: bool,
    pub HasWorkingDir: bool,
    pub HasArguments: bool,
    pub HasIconLocation: bool,
    pub IsUnicode: bool,
    pub ForceNoLinkInfo: bool,
    pub HasExpString: bool,
    pub RunInSeparateProcess: bool,
    pub Unused1: bool,
    pub HasDarwinID: bool,
    pub RunAsUser: bool,
    pub HasExpIcon: bool,
    pub NoPidlAlias: bool,
    pub Unused2: bool,
    pub RunWithShimLayer: bool,
    pub ForceNoLinkTrack: bool,
    pub EnableTargetMetadata: bool,
    pub DisableLinkPathTracking: bool,
    pub DisableKnownFolderTracking: bool,
    pub DisableKnownFolderAlias: bool,
    pub AllowLinkToLink: bool,
    pub UnaliasOnSave: bool,
    pub PreferEnvironmentPath: bool,
    pub KeepLocalIDListForUNCTarget: bool
}

impl LinkFlags{
    pub fn from_u32(flags: u32) -> Result<LinkFlags>
    {
        Ok(
            LinkFlags{
                HasLinkTargetIDList: flags & 0x1000000 != 0,
                HasLinkInfo: flags & 0x2000000 != 0,
                HasName: flags & 0x4000000 != 0,
                HasRelativePath: flags & 0x8000000 != 0,
                HasWorkingDir: flags & 0x10000000 != 0,
                HasArguments: flags & 0x20000000 != 0,
                HasIconLocation: flags & 0x40000000 != 0,
                IsUnicode: flags & 0x80000000 != 0,
                ForceNoLinkInfo: flags & 0x10000 != 0,
                HasExpString: flags & 0x20000 != 0,
                RunInSeparateProcess: flags & 0x40000 != 0,
                Unused1: flags & 0x80000 != 0,
                HasDarwinID: flags & 0x100000 != 0,
                RunAsUser: flags & 0x200000 != 0,
                HasExpIcon: flags & 0x400000 != 0,
                NoPidlAlias: flags & 0x800000 != 0,
                Unused2: flags & 0x100 != 0,
                RunWithShimLayer: flags & 0x200 != 0,
                ForceNoLinkTrack: flags & 0x400 != 0,
                EnableTargetMetadata: flags & 0x800 != 0,
                DisableLinkPathTracking: flags & 0x1000 != 0,
                DisableKnownFolderTracking: flags & 0x2000 != 0,
                DisableKnownFolderAlias: flags & 0x4000 != 0,
                AllowLinkToLink: flags & 0x8000 != 0,
                UnaliasOnSave: flags & 0x1 != 0,
                PreferEnvironmentPath: flags & 0x2 != 0,
                KeepLocalIDListForUNCTarget: flags & 0x4 != 0
            }
        )
    }
    
}


impl Display for LinkFlags{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = vec![];
        if self.HasLinkTargetIDList {
            result.push("HasLinkTargetIDList");
        }
        if self.HasLinkInfo {
            result.push("HasLinkInfo");
        }
        if self.HasName {
            result.push("HasName");
        }
        if self.HasRelativePath {
            result.push("HasRelativePath");
        }
        if self.HasWorkingDir {
            result.push("HasWorkingDir");
        }
        if self.HasArguments {
            result.push("HasArguments");
        }
        if self.HasIconLocation {
            result.push("HasIconLocation");
        }
        if self.IsUnicode {
            result.push("IsUnicode");
        }
        if self.ForceNoLinkInfo {
            result.push("ForceNoLinkInfo");
        }
        if self.HasExpString {
            result.push("HasExpString");
        }
        if self.RunInSeparateProcess {
            result.push("RunInSeparateProcess");
        }
        if self.Unused1 {
            result.push("Unused1");
        }
        if self.HasDarwinID {
            result.push("HasDarwinID");
        }
        if self.RunAsUser {
            result.push("RunAsUser");
        }
        if self.HasExpIcon {
            result.push("HasExpIcon");
        }
        if self.NoPidlAlias {
            result.push("NoPidlAlias");
        }
        if self.Unused2 {
            result.push("Unused2");
        }
        if self.RunWithShimLayer {
            result.push("RunWithShimLayer");
        }
        if self.ForceNoLinkTrack {
            result.push("ForceNoLinkTrack");
        }
        if self.EnableTargetMetadata {
            result.push("EnableTargetMetadata");
        }
        if self.DisableLinkPathTracking {
            result.push("DisableLinkPathTracking");
        }
        if self.DisableKnownFolderTracking {
            result.push("DisableKnownFolderTracking");
        }
        if self.DisableKnownFolderAlias {
            result.push("DisableKnownFolderAlias");
        }
        if self.AllowLinkToLink {
            result.push("AllowLinkToLink");
        }
        if self.UnaliasOnSave {
            result.push("UnaliasOnSave");
        }
        if self.PreferEnvironmentPath {
            result.push("PreferEnvironmentPath");
        }
        if self.KeepLocalIDListForUNCTarget {
            result.push("KeepLocalIDListForUNCTarget");
        }
        write!(f, "{}",result.join(","))
    }
}

impl Serialize for LinkFlags {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_some(&self.to_string().split(",").collect::<Vec<&str>>())
    }
}

/* #endregion */

/* #region  ShellLinkHeader Struct Implementation */

#[derive(Debug,Serialize)]
enum ShowCommandOptions{
    SHOWNORMAL,
    SHOWMAXIMIZED,
    SHOWMINNOACTIVE,
    UNKOWN
}

impl ShowCommandOptions {
    fn from_u32(s: u32) -> ShowCommandOptions{
        match s {
            1 => ShowCommandOptions::SHOWNORMAL,
            3 => ShowCommandOptions::SHOWMAXIMIZED,
            7 => ShowCommandOptions::SHOWMINNOACTIVE,
            _ => ShowCommandOptions::UNKOWN,
        }
    }
}

/// The ShellLinkHeader structure contains identification information, timestamps, and flags that specify the presence of optional structures
#[derive(Debug, Serialize)]
pub struct ShellLinkHeader {
    #[serde(skip_serializing)]
    pub header_size: u32,
    #[serde(skip_serializing)]
    pub guid: Guid,
    #[serde(skip_serializing)]
    pub flags: LinkFlags,
    pub file_attr: Vec<FileAttributesFlags>,
    pub mtime: FileTime,
    pub atime: FileTime,
    pub ctime: FileTime,
    pub file_size: u32,
    #[serde(skip_serializing)]
    pub icon_index: u32,
    #[serde(skip_serializing)]
    sc: ShowCommandOptions,
    #[serde(skip_serializing)]
    pub hot_key: u16,
    #[serde(skip_serializing)]
    pub reserved0: u16,
    #[serde(skip_serializing)]
    pub reserved1: u32,
    #[serde(skip_serializing)]
    pub reserved2: u32
}

impl ShellLinkHeader{
    pub fn from_buffer(buf: &[u8]) -> Result<ShellLinkHeader>
    {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<ShellLinkHeader>
    {
        let header_size = r.read_u32::<LittleEndian>()?;
        let guid = Guid::from_reader(r)?;
        let flags = LinkFlags::from_u32(r.read_u32::<BigEndian>()?)?;
        let file_attr = FileAttributesFlags::from_u32(r.read_u32::<LittleEndian>()?);
        let ctime = FileTime::new(r.read_u64::<LittleEndian>()?);
        let atime = FileTime::new(r.read_u64::<LittleEndian>()?);
        let mtime = FileTime::new(r.read_u64::<LittleEndian>()?);
        let file_size = r.read_u32::<LittleEndian>()?;
        let icon_index = r.read_u32::<LittleEndian>()?;
        let sc = ShowCommandOptions::from_u32(r.read_u32::<LittleEndian>()?);
        let hot_key = r.read_u16::<LittleEndian>()?;
        let reserved0 = r.read_u16::<LittleEndian>()?;
        let reserved1 = r.read_u32::<LittleEndian>()?;
        let reserved2 = r.read_u32::<LittleEndian>()?;

        Ok(
            ShellLinkHeader{
                header_size,
                guid,
                flags,
                file_attr,
                ctime,
                atime,
                mtime,
                file_size,
                icon_index,
                sc,
                hot_key,
                reserved0,
                reserved1,
                reserved2,
            }
        )
    }
}
/* #endregion */