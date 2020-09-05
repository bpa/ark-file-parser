use crate::io::{Name, Reader};
use crate::object::Names;
use crate::properties::{Properties, Property, Value};
use std::collections::hash_map::Entry;
use std::io::{Result, SeekFrom};
use std::rc::Rc;

impl<'a> Properties {
    pub fn read(
        &mut self,
        file: &mut dyn Reader,
        names: &Rc<Names>,
        properties_offset: u64,
    ) -> Result<()> {
        let here = file.seek(SeekFrom::Current(0))?;
        file.seek(SeekFrom::Start(properties_offset))?;
        loop {
            let id = file.read_u32()? as usize;
            if &names[id] == "None" {
                break;
            }
            let instance = file.read_u32()?;
            let var_type = file.read_u32()? as usize;
            file.seek(SeekFrom::Current(4))?;
            let data_size = file.read_u32()?;
            let ind = file.read_u32()?;
            let value = match &names[var_type] {
                "ArrayProperty" => read_array(file, data_size)?,
                "BoolProperty" => Value::Bool(file.read_u8()? != 0),
                "ByteProperty" => read_byte(file, names)?,
                "DoubleProperty" => Value::Double(file.read_f64()?),
                "FloatProperty" => Value::Float(file.read_f32()?),
                "Int16Property" => Value::Int16(file.read_i16()?),
                "Int8Property" => Value::Int8(file.read_i8()?),
                "IntProperty" => Value::Int(file.read_i32()?),
                "NameProperty" => Value::Name(file.read_name()?),
                "ObjectProperty" => read_object(file, data_size)?,
                "StrProperty" => Value::String(file.read_str()?),
                "StructProperty" => read_struct(file, data_size)?,
                "TextProperty" => panic!("Implement text"),
                "UInt16Property" => Value::UInt16(file.read_u16()?),
                "UInt32Property" => Value::UInt32(file.read_u32()?),
                "UInt64Property" => Value::UInt64(file.read_u64()?),
                _ => panic!("Unknown type {}", &names[var_type]),
            };
            let property = Property {
                name: Name { id, instance },
                ind,
                value,
            };
            match self.props.entry(id) {
                Entry::Occupied(mut e) => e.get_mut().push(property),
                Entry::Vacant(e) => {
                    e.insert(vec![property]);
                    self.set.insert(names[id].into());
                }
            }
        }
        file.seek(SeekFrom::Start(here))?;
        Ok(())
    }
}

pub fn read_array(file: &mut dyn Reader, size: u32) -> Result<Value> {
    let array_type = file.read_name()?;
    file.seek(SeekFrom::Current(size as i64))?;
    Ok(Value::Array(array_type))
}

fn read_byte(file: &mut dyn Reader, names: &Names) -> Result<Value> {
    let enum_name = file.read_name()?;
    if &names[enum_name.id] == "None" {
        Ok(Value::Byte(file.read_u8()?))
    } else {
        //Its an enum, not a byte
        Ok(Value::Enum(enum_name, file.read_name()?))
    }
}

fn read_object(file: &mut dyn Reader, data_size: u32) -> Result<Value> {
    match data_size {
        4 => Ok(Value::Int(file.read_i32()?)),
        x if x >= 8 => {
            let prop_type = file.read_u32()?;
            if prop_type == 0 {
                // ID
                Ok(Value::Int(file.read_i32()?))
            } else if prop_type == 1 {
                // Path
                let name = file.read_name()?;
                Ok(Value::Name(name))
            } else {
                panic!("Unknown object prop type {}", prop_type);
            }
        }
        _ => {
            panic!("Reading ojbect with data size {}", data_size);
        }
    }
}

fn read_struct(file: &mut dyn Reader, data_size: u32) -> Result<Value> {
    let name = file.read_name()?;
    file.seek(SeekFrom::Current(data_size as i64))?;
    Ok(Value::Struct(name, data_size))
}
