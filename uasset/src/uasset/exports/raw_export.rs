use std::io::{Cursor, Read};
use crate::uasset::Asset;
use crate::uasset::error::Error;
use crate::uasset::exports::ExportUnknownTrait;
use crate::uasset::exports::unknown_export::UnknownExport;

use super::ExportNormalTrait;

pub struct RawExport {
    unknown_export: UnknownExport,

    data: Vec<u8>
}

impl ExportNormalTrait for RawExport {
    fn get_normal_export<'a>(&'a self) -> Option<&'a super::normal_export::NormalExport> {
        None
    }


    fn get_normal_export_mut<'a>(&'a mut self) -> Option<&'a mut super::normal_export::NormalExport> {
        None
    }
}

impl ExportUnknownTrait for RawExport {
    fn get_unknown_export<'a>(&'a self) -> &'a UnknownExport {
        &self.unknown_export
    }

    fn get_unknown_export_mut<'a>(&'a mut self) -> &'a mut UnknownExport {
        &mut self.unknown_export
    }
}

impl RawExport {
    pub fn from_unk(unk: UnknownExport, asset: &mut Asset) -> Result<Self, Error> {
        let mut cursor = &mut asset.cursor;
        let mut data = Vec::with_capacity(unk.serial_size as usize);
        cursor.read_exact(&mut data)?;

        Ok(RawExport {
            unknown_export: unk,
            data
        })
    }
}