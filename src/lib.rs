#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
pub mod extra_data;
pub mod link_info;
mod link_target_id_list;
pub mod shell_link_header;

use extra_data::{ExtraData, ExtraDataTypes};
use getset::Getters;
use link_info::LinkInfo;
use link_target_id_list::LinkTargetIDList;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use shell_link_header::ShellLinkHeader;

use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    fs,
    io::{Cursor, Read, Seek},
};
use winparsingtools::{
    structs::StringData,
    traits::{Normalize, Path}, ReaderError,
};

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct LnkFileMetaData {
    full_path: String,
    mtime: DateTime<Utc>,
    atime: DateTime<Utc>,
    ctime: DateTime<Utc>,
}

impl LnkFileMetaData {
    fn from_path(path: &str) -> Result<Self, ReaderError> {
        let file_metadata = fs::metadata(path)?;
        let full_path = match fs::canonicalize(path) {
            Ok(path_buf) => path_buf
                .to_str()
                .ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Can not Read full_path for '{}'", path),
                ))?
                .to_string()
                .replace("\\\\?\\", ""),
            Err(_) => path.to_string(),
        };
        let mtime: DateTime<Utc> = DateTime::from(file_metadata.created()?);
        let atime: DateTime<Utc> = DateTime::from(file_metadata.accessed()?);
        let ctime: DateTime<Utc> = DateTime::from(file_metadata.modified()?);
        Ok(Self {
            full_path,
            mtime,
            ctime,
            atime,
        })
    }
}

impl Serialize for LnkFileMetaData {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("LnkFileMetaData", 4)?;
        state.serialize_field("full_path", &self.full_path)?;
        state.serialize_field(
            "mtime",
            &format!("{}", self.mtime.format("%Y-%m-%dT%H:%M:%SZ")),
        )?;
        state.serialize_field(
            "atime",
            &format!("{}", self.atime.format("%Y-%m-%dT%H:%M:%SZ")),
        )?;
        state.serialize_field(
            "ctime",
            &format!("{}", self.ctime.format("%Y-%m-%dT%H:%M:%SZ")),
        )?;
        state.end()
    }
}

/// Reads LNK file and determine its parts then parses them
#[derive(Debug, Serialize, Getters)]
#[getset(get = "pub with_prefix")]
pub struct LNKParser {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_full_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lnk_file_metadata: Option<LnkFileMetaData>,
    shell_link_header: ShellLinkHeader,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_target_id_list: Option<LinkTargetIDList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_info: Option<LinkInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_string: Option<StringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_path: Option<StringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<StringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command_line_arguments: Option<StringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_location: Option<StringData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_data: Option<ExtraData>,
}

impl LNKParser {
    /// Parse LNK file from path.
    /// # Example
    /// ```
    ///# use lnk_parser::LNKParser;
    /// fn main(){
    ///     let lnk_file = LNKParser::from_path("sample.lnk");
    ///     println!("{:?}", lnk_file);
    /// }
    /// ```
    pub fn from_path(path: &str) -> Result<Self, ReaderError> {
        let lnk_file_metadata = LnkFileMetaData::from_path(path)?;
        let mut lnk_file_reader = fs::File::open(path)?;
        let mut lnk_parser = Self::from_reader(&mut lnk_file_reader)?;
        lnk_parser.lnk_file_metadata = Some(lnk_file_metadata);
        Ok(lnk_parser)
    }
    /// Parse the LNK file data from buffer
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }
    /// Parse LNK file from an instance that implement `Read` & `Seek` traits.
    /// # Example
    /// ```
    ///# use lnk_parser::LNKParser;
    /// use std::fs::File;
    /// fn main(){
    ///     // Open the LNK file
    ///     let mut file = File::open("samples/WIN7/6.1_7601/network_share.lnk").unwrap();
    ///     // Pass the `File` instance to `from_reader` function.
    ///     // `std::fs::File` implements `Read` & `Seek` traits.
    ///     let lnk_file = LNKParser::from_reader(&mut file);
    ///     println!("{:?}", lnk_file);
    /// }
    /// ```
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        let shell_link_header = ShellLinkHeader::from_reader(r)?;
        let mut link_target_id_list = None;
        let mut link_info = None;
        let mut name_string = None;
        let mut relative_path = None;
        let mut working_dir = None;
        let mut command_line_arguments = None;
        let mut icon_location = None;

        if shell_link_header.flags.HasLinkTargetIDList {
            link_target_id_list = Some(LinkTargetIDList::from_reader(r)?);
        }
        if shell_link_header.flags.HasLinkInfo {
            link_info = Some(LinkInfo::from_reader(r)?);
        }
        if shell_link_header.flags.HasName {
            name_string = Some(StringData::from_reader(r)?);
        }
        if shell_link_header.flags.HasRelativePath {
            relative_path = Some(StringData::from_reader(r)?);
        }
        if shell_link_header.flags.HasWorkingDir {
            working_dir = Some(StringData::from_reader(r)?);
        }
        if shell_link_header.flags.HasArguments {
            command_line_arguments = Some(StringData::from_reader(r)?);
        }
        if shell_link_header.flags.HasIconLocation {
            icon_location = Some(StringData::from_reader(r)?);
        }

        let extra_data = match ExtraData::from_reader(r) {
            Ok(d) => Some(d),
            Err(_) => None,
        };

        let mut lnk_parser = Self {
            shell_link_header,
            link_target_id_list,
            link_info,
            name_string,
            relative_path,
            working_dir,
            command_line_arguments,
            icon_location,
            extra_data,
            lnk_file_metadata: None,
            target_full_path: None,
        };
        lnk_parser.target_full_path = lnk_parser.path();

        Ok(lnk_parser)
    }
}

impl Path for LNKParser {
    fn path(&self) -> Option<String> {
        match &self.link_info {
            Some(link_info) => match link_info.path() {
                Some(link_info_path) => Some(link_info_path),
                None => match &self.link_target_id_list {
                    Some(link_target_id_list) => link_target_id_list.path(),
                    None => None,
                },
            },
            None => match &self.link_target_id_list {
                Some(link_target_id_list) => link_target_id_list.path(),
                None => None,
            },
        }
    }
}

impl Normalize for LNKParser {
    fn normalize(&self) -> HashMap<String, String> {
        let mut fields: HashMap<String, String> = HashMap::new();
        let mut target_hostname = String::new();

        let target_full_path = match &self.path() {
            Some(path) => path.to_owned(),
            None => String::new(),
        };

        let target_modification_time = self.shell_link_header.mtime.to_string();
        let target_access_time = self.shell_link_header.atime.to_string();
        let target_creation_time = self.shell_link_header.ctime.to_string();

        let target_size = self.shell_link_header.file_size.to_string();

        match &self.extra_data {
            Some(extra_data) => {
                extra_data.extra_data_blocks.iter().find(|&edb| match edb {
                    ExtraDataTypes::Tracker(tracker) => {
                        target_hostname = tracker.machine_id.to_owned();
                        true
                    }
                });
            }
            None => {}
        };

        let lnk_full_path = match &self.lnk_file_metadata {
            Some(lnk_file_metadata) => lnk_file_metadata.full_path.to_owned(),
            None => String::new(),
        };

        let lnk_modification_time = match &self.lnk_file_metadata {
            Some(lnk_file_metadata) => lnk_file_metadata
                .mtime
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
            None => String::new(),
        };

        let lnk_access_time = match &self.lnk_file_metadata {
            Some(lnk_file_metadata) => lnk_file_metadata
                .atime
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
            None => String::new(),
        };

        let lnk_creation_time = match &self.lnk_file_metadata {
            Some(lnk_file_metadata) => lnk_file_metadata
                .ctime
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
            None => String::new(),
        };

        fields.insert("target_full_path".to_string(), target_full_path);
        fields.insert(
            "target_modification_time".to_string(),
            target_modification_time,
        );
        fields.insert("target_access_time".to_string(), target_access_time);
        fields.insert("target_creation_time".to_string(), target_creation_time);
        fields.insert("target_size".to_string(), target_size);
        fields.insert("target_hostname".to_string(), target_hostname);
        fields.insert("lnk_full_path".to_string(), lnk_full_path);
        fields.insert("lnk_modification_time".to_string(), lnk_modification_time);
        fields.insert("lnk_access_time".to_string(), lnk_access_time);
        fields.insert("lnk_creation_time".to_string(), lnk_creation_time);
        fields
    }
}
