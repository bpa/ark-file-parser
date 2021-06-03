use crate::object::{Name, Names};
use crate::properties::Properties;
use crate::Object;
use crate::{io::Reader, Location};
use std::io::{Result, SeekFrom};
use std::rc::Rc;

pub struct CryopodParser {
    pub objects: Vec<Object>,
}

impl CryopodParser {
    pub fn read(file: &mut dyn Reader, names: &Rc<Names>) -> Result<Self> {
        let count = file.read_i32()?;
        let mut objects = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let guid = file.read_u128()?;
            let _name = file.read_str()?;
            let is_item = file.read_bool()?;

            let extra_class_count = file.read_i32()?;
            for _ in 0..extra_class_count {
                file.read_str()?;
            }
            // println!("{:?}", location);
            file.seek(SeekFrom::Current(8))?;

            let location = if file.read_bool()? {
                Some(Location::read(file)?)
            } else {
                None
            };

            let object_properties_offset = file.read_i32()? as u64;
            let mut properties = Properties::new();
            let next_object = file.seek(SeekFrom::Current(4))?;

            file.seek(SeekFrom::Start(object_properties_offset))?;
            properties.read(file, names)?;
            file.seek(SeekFrom::Start(next_object))?;

            objects.push(Object::new(
                guid,
                Name { id: 0, instance: 0 },
                is_item,
                location,
                properties,
                names.clone(),
            ));
        }
        Ok(CryopodParser { objects })
    }
}
