use crate::io::{ArrayReader, Reader};
use crate::object::{Names, Object};
use crate::properties::{Properties, Value};
use crate::{Entry, Location};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{Result, SeekFrom, Write};
use std::rc::Rc;

use super::CryopodParser;

pub struct ArkParser {
    _class_offset: u64,
    pub names: Rc<Names>,
    entries: Vec<Entry>,
    pub map: String,
}

impl ArkParser {
    pub fn read(file: &mut dyn Reader) -> Result<Self> {
        let (_version, names_offset, properties_offset) = read_header(file)?;
        let map = skip_binary_data_names(file)?;
        skip_embedded_binary_data(file)?;
        skip_data_files_object_map(file)?;
        let names = Rc::new(Names::new(file, names_offset)?);
        let mut objects = read_objects(file, &names, properties_offset)?;
        let mut frozen_dinos = uncryopod_dinos(&objects, &names);
        objects.append(&mut frozen_dinos);
        let objects = Rc::new(objects);
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

        Ok(ArkParser {
            _class_offset: names_offset,
            names,
            entries,
            map,
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

fn read_header(file: &mut dyn Reader) -> Result<(i16, u64, u64)> {
    let version = file.read_i16()?;
    if version < 5 || version > 9 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unsupported file version {}", version),
        ));
    }

    if version > 6 {
        let _hibernation_offset = file.read_u32()? as u64;
        let zero_value = file.read_i32()?;
        if zero_value != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Found non zero value in header",
            ));
        }
    }

    let names_offset = file.read_u32()? as u64;
    let properties_offset = file.read_i32()? as u64;
    let _game_time = file.read_f32();

    if version > 8 {
        let _save_count = file.read_u32()?;
    }

    // let classes_offset = file.read_i32()? as u64;
    // file.seek(SeekFrom::Current(8))?;
    Ok((version, names_offset, properties_offset))
}

fn skip_binary_data_names(file: &mut dyn Reader) -> Result<String> {
    let count = file.read_i32()?;
    let map = file.read_str()?;
    for _ in 1..count {
        file.skip_str()?;
    }
    Ok(map)
}

fn skip_embedded_binary_data(file: &mut dyn Reader) -> Result<()> {
    let data_count = file.read_i32()?;
    for _ in 0..data_count {
        file.skip_str()?; //Path
        let parts = file.read_i32()?;
        for _ in 0..parts {
            let blobs = file.read_i32()?;
            for _ in 0..blobs {
                let blob_size = file.read_i32()? * 4;
                file.seek(SeekFrom::Current(blob_size as i64))?;
            }
        }
    }
    Ok(())
}

fn skip_data_files_object_map(file: &mut dyn Reader) -> Result<()> {
    let entries = file.read_i32()?;
    for _ in 0..entries {
        file.seek(SeekFrom::Current(4 as i64))?;
        let count = file.read_u32()?;
        for _ in 0..count {
            file.skip_str()?;
        }
    }
    Ok(())
}

fn uncryopod_dinos(objects: &Vec<Object>, names: &Rc<Names>) -> Vec<Object> {
    let cryopod = names
        .get_name_id("PrimalItem_WeaponEmptyCryopod_C")
        .unwrap();
    let custom_item_data = names.get_name_id("CustomItemDatas").unwrap();
    let heirarchy: Vec<usize> = vec!["CustomDataBytes", "ByteArrays", "Bytes"]
        .iter()
        .map(|name| *names.get_name_id(name).or(Some(&0)).unwrap())
        .collect();

    // let frozen_dinos = objects
    objects
        .iter()
        .filter(|o| o.name.id == *cryopod && o.properties.contains_id(custom_item_data))
        .for_each(|o| {
            let custom_data = o
                .properties
                .props
                .get(custom_item_data)
                .unwrap()
                .first()
                .unwrap();
            let mut value = &custom_data.value;
            for id in &heirarchy {
                if let Value::Properties(properties) = value {
                    value = &properties.props.get(&id).unwrap().first().unwrap().value;
                }
            }
            if let Value::ArrayOfU8(data) = value {
                let mut f = File::create("cryo.dat").unwrap();
                f.write_all(&data.as_slice()).unwrap();
                let mut frozen_data = ArrayReader::from(data);
                let parser = CryopodParser::read(&mut frozen_data, names).unwrap();
                let frozen_objects = parser.objects;
                serde_json::to_writer_pretty(std::io::stderr(), &frozen_objects).unwrap();
            }
        });
    // .collect();
    Vec::new()
}
