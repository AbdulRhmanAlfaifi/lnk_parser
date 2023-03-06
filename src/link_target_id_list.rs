use std::io::{Read, Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use winparsingtools::{
    traits::Path,
    structs::shell_items::{IDList, ShellItem}, ReaderError
};
use serde::Serialize;

/// The [LinkTargetIDList](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/881d7a83-07a5-4702-93e3-f9fc34c3e1e4) structure specifies the target of the link.
#[derive(Debug, Serialize)]
pub struct LinkTargetIDList {
    #[serde(skip_serializing)]
    pub size: u16,
    pub id_list: IDList
}

impl LinkTargetIDList {
    #[allow(dead_code)]
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        let size = r.read_u16::<LittleEndian>()?;
        let mut id_list_data = vec![0;size as usize];
        r.read_exact(&mut id_list_data)?;
        let id_list = IDList::from_buffer(&id_list_data)?;
        Ok(Self {
            size,
            id_list
        })
    }

    pub fn items(&self) -> std::slice::Iter<'_, ShellItem> {
        self.id_list.items()
    }
}

impl Path for LinkTargetIDList {
    fn path(&self) -> Option<String> {
        self.id_list.path()
    }
}
