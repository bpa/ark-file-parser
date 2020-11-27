use super::location::Location;
use super::names::Names;
use super::Entry;
use crate::ark::Ark;
use crate::io::{MMappedReader, Reader};
use crate::object::Object;
use crate::properties::Properties;
use std::io::{Result, SeekFrom};
use std::rc::Rc;

pub struct ArkSave {
    _class_offset: u64,
    pub names: Rc<Names>,
    entries: Vec<Entry>,
    pub map: Ark,
}

impl ArkSave {
    pub fn read(filename: &str) -> Result<Self> {
        let mut file = MMappedReader::open(filename)?;

        let (class_offset, properties_offset) = read_header(&mut file)?;
        skip_binary_data_names(&mut file)?;
        skip_embedded_binary_data(&mut file)?;
        skip_unknown_data(&mut file)?;
        let names = Rc::new(Names::new(&mut file, class_offset)?);
        let objects = Rc::new(read_objects(&mut file, &names, properties_offset)?);
        let entries = objects
            .iter()
            .enumerate()
            .map(|(i, o)| Entry {
                object_type: o.object_type,
                objects: objects.clone(),
                object: i,
                inventory: o.inventory_component,
                status: o.status_component,
            })
            .collect();

        Ok(ArkSave {
            _class_offset: class_offset,
            map: Ark::new(50.0, 8000.0, 50.0, 8000.0),
            names,
            entries,
        })
    }

    pub fn get_name(&self, id: usize) -> &str {
        &self.names[id]
    }

    pub fn get_name_id(&self, name: &str) -> Option<&usize> {
        self.names.get_name_id(name)
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

pub fn read_objects<'a>(
    file: &mut dyn Reader,
    names: &Rc<Names>,
    properties_offset: u64,
) -> Result<Vec<Object>> {
    let object_count = file.read_i32()?;
    let mut objects = Vec::with_capacity(object_count as usize);
    for _ in 0..object_count {
        let guid = file.read_u128()?;
        let name = file.read_name()?;
        let is_item = file.read_bool()?;

        let extra_class_count = file.read_i32()?;
        for _ in 0..extra_class_count {
            file.read_name()?;
        }
        file.seek(SeekFrom::Current(8))?;

        let location = if file.read_bool()? {
            Some(Location::read(file)?)
        } else {
            None
        };

        let object_properties_offset = file.read_i32()? as u64;
        let mut properties = Properties::new();
        let next_object = file.seek(SeekFrom::Current(4))?;

        file.seek(SeekFrom::Start(
            properties_offset + object_properties_offset,
        ))?;
        properties.read(file, names)?;
        file.seek(SeekFrom::Start(next_object))?;

        objects.push(Object::new(
            guid,
            name,
            is_item,
            location,
            properties,
            names.clone(),
        ));
    }
    Ok(objects)
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
