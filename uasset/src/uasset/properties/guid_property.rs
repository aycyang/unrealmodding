use std::io::{Cursor, Error, ErrorKind, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{uasset::{unreal_types::{Guid, FName}, cursor_ext::CursorExt}, optional_guid};

#[derive(Debug)]
pub struct GuidProperty {
    name: FName,
    property_guid: Option<Guid>,
    value: Guid
}

impl GuidProperty {
    pub fn new(name: FName, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        let property_guid = optional_guid!(cursor, include_header);
        let mut value = [0u8; 16];
        cursor.read_exact(&mut value)?;
        Ok(GuidProperty {
            name,
            property_guid,
            value
        })
    }
}