use crate::ark::Ark;
use crate::io::{MMappedReader, Reader};
use crate::location::Location;
use crate::names::Names;
use crate::object::Object;
use crate::properties::Properties;
use std::io::{Result, Seek, SeekFrom};

pub struct ArkSave {
    file: MMappedReader,
    class_offset: u64,
    objects_offset: u64,
    properties_offset: u64,
    _map: Ark,
}

impl ArkSave {
    pub fn open(filename: &str) -> Result<Self> {
        let mut file = MMappedReader::open(filename)?;

        let (class_offset, properties_offset) = read_header(&mut file)?;
        skip_binary_data_names(&mut file)?;
        skip_embedded_binary_data(&mut file)?;
        skip_unknown_data(&mut file)?;
        let objects_offset = file.seek(SeekFrom::Current(0))?;

        Ok(ArkSave {
            file,
            class_offset,
            objects_offset,
            properties_offset,
            _map: Ark::new(50.0, 8000.0, 50.0, 8000.0),
        })
    }

    pub fn read_names(&mut self) -> Result<Names> {
        Names::new(&mut self.file, self.class_offset)
    }

    pub fn read_objects<'a>(&mut self, names: &Names) -> Result<Vec<Object<'a>>> {
        let object_count = self.file.read_i32()?;
        let mut objects = Vec::with_capacity(object_count as usize);
        for _ in 0..object_count {
            let guid = self.file.read_u128()?;
            let name = self.file.read_name()?;
            let is_item = self.file.read_bool()?;

            let extra_class_count = self.file.read_i32()?;
            let mut extra_classes = Vec::with_capacity(extra_class_count as usize);
            for _ in 0..extra_class_count {
                extra_classes.push(self.file.read_name()?);
            }
            self.file.seek(SeekFrom::Current(8))?;

            let location = if self.file.read_bool()? {
                Some(Location::read(&mut self.file)?)
            } else {
                None
            };

            let object_properties_offset = self.file.read_i32()? as u64;
            let skip = &names[name.id] == "InstancedFoliageActor";
            let mut properties = Properties::new();
            if !skip {
                properties.read(
                    &mut self.file,
                    names,
                    self.properties_offset + object_properties_offset,
                )?
            };

            self.file.seek(SeekFrom::Current(4))?;

            objects.push(Object::new(
                guid,
                name,
                is_item,
                extra_classes,
                location,
                properties,
                names,
            ));
        }
        Ok(objects)
    }
}

fn read_header(file: &mut dyn Reader) -> Result<(u64, u64)> {
    let _version = file.read_i16()?;
    file.seek(SeekFrom::Current(8))?;
    let classes_offset = file.read_i32()? as u64;
    let properties_offset = file.read_i32()? as u64;
    file.seek(SeekFrom::Current(8))?;
    Ok((classes_offset, properties_offset))
}

fn skip_binary_data_names(file: &mut dyn Reader) -> Result<()> {
    //TODO: Set map based on first binary name
    let count = file.read_i32()?;
    for _ in 0..count {
        let size = file.read_i32()?;
        file.seek(SeekFrom::Current(size as i64))?;
    }
    Ok(())
}

fn skip_embedded_binary_data(file: &mut dyn Reader) -> Result<()> {
    let parts = file.read_i32()?;
    for _ in 0..parts {
        let blobs = file.read_i32()?;
        for _ in 0..blobs {
            let blob_size = file.read_i32()?;
            file.seek(SeekFrom::Current(blob_size as i64))?;
        }
    }
    Ok(())
}

fn skip_unknown_data(file: &mut dyn Reader) -> Result<()> {
    let entries = file.read_i32()?;
    for _ in 0..entries {
        file.seek(SeekFrom::Current(8 as i64))?;
        let str_len = file.read_i32()? as i64;
        file.seek(SeekFrom::Current(str_len))?;
    }
    Ok(())
}
