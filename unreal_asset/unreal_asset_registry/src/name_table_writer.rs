//! Asset registry NameTableWriter
use std::io::{self, Seek, SeekFrom, Write};

use unreal_asset_base::{
    containers::{IndexedMap, NameMap, SharedResource},
    custom_version::{CustomVersion, CustomVersionTrait},
    engine_version::EngineVersion,
    object_version::{ObjectVersion, ObjectVersionUE5},
    reader::{ArchiveTrait, ArchiveType, ArchiveWriter, PassthroughArchiveWriter},
    types::{FName, PackageIndex},
    unversioned::Usmap,
    Import,
};

/// Used to write NameTable entries by modifying the behavior
/// of some of the value write methods.
pub struct NameTableWriter<'writer, Writer: ArchiveWriter> {
    /// Writer
    writer: &'writer mut Writer,
    /// Name map
    name_map: SharedResource<NameMap>,
}

impl<'writer, Writer: ArchiveWriter> NameTableWriter<'writer, Writer> {
    /// Create a new `NameTableWriter` instance from another `Writer` and a name map
    pub fn new(writer: &'writer mut Writer, name_map: SharedResource<NameMap>) -> Self {
        NameTableWriter { writer, name_map }
    }
}

impl<'writer, Writer: ArchiveWriter> ArchiveTrait for NameTableWriter<'writer, Writer> {
    #[inline(always)]
    fn get_archive_type(&self) -> ArchiveType {
        self.writer.get_archive_type()
    }

    fn get_custom_version<T>(&self) -> CustomVersion
    where
        T: CustomVersionTrait + Into<i32>,
    {
        self.writer.get_custom_version::<T>()
    }

    fn has_unversioned_properties(&self) -> bool {
        self.writer.has_unversioned_properties()
    }

    fn use_event_driven_loader(&self) -> bool {
        self.writer.use_event_driven_loader()
    }

    fn position(&mut self) -> u64 {
        self.writer.position()
    }

    fn set_position(&mut self, pos: u64) -> io::Result<()> {
        self.writer.set_position(pos)
    }

    fn get_name_map(&self) -> SharedResource<NameMap> {
        self.name_map.clone()
    }

    fn get_array_struct_type_override(&self) -> &IndexedMap<String, String> {
        self.writer.get_array_struct_type_override()
    }

    fn get_map_key_override(&self) -> &IndexedMap<String, String> {
        self.writer.get_map_key_override()
    }

    fn get_map_value_override(&self) -> &IndexedMap<String, String> {
        self.writer.get_map_value_override()
    }

    fn get_engine_version(&self) -> EngineVersion {
        self.writer.get_engine_version()
    }

    fn get_object_version(&self) -> ObjectVersion {
        self.writer.get_object_version()
    }

    fn get_object_version_ue5(&self) -> ObjectVersionUE5 {
        self.writer.get_object_version_ue5()
    }

    fn get_mappings(&self) -> Option<&Usmap> {
        self.writer.get_mappings()
    }

    fn get_parent_class_export_name(&self) -> Option<FName> {
        self.writer.get_parent_class_export_name()
    }

    fn get_import(&self, index: PackageIndex) -> Option<Import> {
        self.writer.get_import(index)
    }
}

impl<'writer, Writer: ArchiveWriter> PassthroughArchiveWriter for NameTableWriter<'writer, Writer> {
    type Passthrough = Writer;
    fn get_passthrough(&mut self) -> &mut Self::Passthrough {
        self.writer
    }
}

impl<'writer, Writer: ArchiveWriter> Write for NameTableWriter<'writer, Writer> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<'writer, Writer: ArchiveWriter> Seek for NameTableWriter<'writer, Writer> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.writer.seek(pos)
    }
}
