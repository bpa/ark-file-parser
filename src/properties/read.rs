use crate::io::{Name, Reader};
use crate::object::Names;
use crate::properties::{Properties, Property, Value};
use base64::decode;
use std::collections::hash_map::Entry;
use std::io::{Result, SeekFrom};
use std::rc::Rc;

impl<'a> Properties {
    pub fn read(&mut self, file: &mut dyn Reader, names: &Rc<Names>) -> Result<()> {
        loop {
            let id = file.read_u32()? as usize;
            if &names[id] == "None" {
                break;
            }
            let instance = file.read_u32()?;
            let type_id = file.read_u32()? as usize;
            file.seek(SeekFrom::Current(4))?;
            let data_size = file.read_u32()? as usize;
            let ind = file.read_u32()?;
            let value = match type_id {
                _ if type_id == names.array_property => read_array(file, data_size, names)?,
                _ if type_id == names.bool_property => Value::Bool(file.read_u8()? != 0),
                _ if type_id == names.byte_property => read_byte(file, names)?,
                _ if type_id == names.double_property => Value::Double(file.read_f64()?),
                _ if type_id == names.float_property => Value::Float(file.read_f32()?),
                _ if type_id == names.int16_property => Value::Int16(file.read_i16()?),
                _ if type_id == names.int8_property => Value::Int8(file.read_i8()?),
                _ if type_id == names.int_property => Value::Int(file.read_i32()?),
                _ if type_id == names.name_property => Value::Name(file.read_name()?),
                _ if type_id == names.object_property => read_object(file, data_size)?,
                _ if type_id == names.str_property => Value::String(file.read_str()?),
                _ if type_id == names.struct_property => read_struct(file, data_size, names)?,
                _ if type_id == names.text_property => read_text(file, data_size)?,
                _ if type_id == names.uint16_property => Value::UInt16(file.read_u16()?),
                _ if type_id == names.uint32_property => Value::UInt32(file.read_u32()?),
                _ if type_id == names.uint64_property => Value::UInt64(file.read_u64()?),
                _ => panic!("Unknown type {}", &names[type_id]),
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
        Ok(())
    }
}

macro_rules! arr {
    ($file:ident, $method:ident, $arm:path) => {{
        let size = $file.read_i32()? as usize;
        let mut items = Vec::with_capacity(size);
        for _ in 0..size {
            items.push($file.$method()?);
        }
        Ok($arm(items))
    }};
}

fn read_array(file: &mut dyn Reader, size: usize, names: &Rc<Names>) -> Result<Value> {
    let array_type = file.read_name()?;
    let id = array_type.id;
    match id {
        _ if id == names.object_property => {
            let size = file.read_i32()? as usize;
            let mut items = Vec::with_capacity(size);
            for _ in 0..size {
                items.push(match file.read_i32()? {
                    0 => Value::Int(file.read_i32()?),
                    1 => Value::Name(file.read_name()?),
                    t @ _ => panic!("Unsupported object type {}", t),
                });
            }
            Ok(Value::ArrayOfObject(items))
        }
        _ if id == names.struct_property => {
            let end_properties = file.seek(SeekFrom::Current(0))? + size as u64;
            let count = file.read_i32()? as usize;
            let mut items = Vec::with_capacity(count);
            let struct_size = if count == 0 {
                0
            } else {
                (size - 4) / 4 / count
            };
            let struct_type_id = match struct_size {
                1 => names.color_property,
                3 => names.vector_property,
                4 => names.linear_color_property,
                _ => {
                    let mut props = Properties::new();
                    props.read(file, names)?;
                    file.seek(SeekFrom::Start(end_properties))?;
                    return Ok(Value::Properties(props));
                }
            };
            for _ in 0..count {
                items.push(read_struct_value(file, struct_type_id, names)?);
            }
            file.seek(SeekFrom::Start(end_properties))?;
            Ok(Value::ArrayOfStruct(items))
        }
        _ if id == names.uint32_property => arr!(file, read_u32, Value::ArrayOfU32),
        _ if id == names.int_property => arr!(file, read_i32, Value::ArrayOfI32),
        _ if id == names.uint16_property => arr!(file, read_u16, Value::ArrayOfU16),
        _ if id == names.int16_property => arr!(file, read_i16, Value::ArrayOfI16),
        _ if id == names.byte_property => arr!(file, read_u8, Value::ArrayOfU8),
        _ if id == names.int8_property => arr!(file, read_i8, Value::ArrayOfI8),
        _ if id == names.str_property => arr!(file, read_str, Value::ArrayOfStr),
        _ if id == names.uint64_property => arr!(file, read_u64, Value::ArrayOfU64),
        _ if id == names.bool_property => arr!(file, read_bool, Value::ArrayOfBool),
        _ if id == names.float_property => arr!(file, read_f32, Value::ArrayOfF32),
        _ if id == names.double_property => arr!(file, read_f64, Value::ArrayOfF64),
        _ if id == names.name_property => arr!(file, read_f32, Value::ArrayOfF32),
        // _ => panic!("Unknown array type {}", &names[array_type.id]),
        _ => {
            file.seek(SeekFrom::Current(size as i64))?;
            Ok(Value::ArrayOfI32(vec![]))
        }
    }
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

fn read_object(file: &mut dyn Reader, data_size: usize) -> Result<Value> {
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
            panic!("Reading object with data size {}", data_size);
        }
    }
}

fn read_struct(file: &mut dyn Reader, data_size: usize, names: &Rc<Names>) -> Result<Value> {
    let name = file.read_name()?;
    let end = file.seek(SeekFrom::Current(0))? + data_size as u64;
    let struct_value = read_struct_value(file, name.id, names)?;
    file.seek(SeekFrom::Start(end))?;
    Ok(struct_value)
}

fn read_struct_value(
    file: &mut dyn Reader,
    struct_type_id: usize,
    names: &Rc<Names>,
) -> Result<Value> {
    Ok(match struct_type_id {
        _ if struct_type_id == names.vector_property => read_vector_struct(file)?,
        _ if struct_type_id == names.rotator_property => read_vector_struct(file)?,
        _ if struct_type_id == names.vector2d_property => read_vector2d_struct(file)?,
        _ if struct_type_id == names.quat_property => read_quat_struct(file)?,
        _ if struct_type_id == names.color_property => read_color_struct(file)?,
        _ if struct_type_id == names.linear_color_property => read_linear_color_struct(file)?,
        _ if struct_type_id == names.unique_netid_property => read_net_struct(file)?,
        _ => {
            let mut properties = Properties::new();
            properties.read(file, names)?;
            Value::Properties(properties)
        }
    })
}

fn read_text(file: &mut dyn Reader, data_size: usize) -> Result<Value> {
    let mut buf = vec![0; data_size];
    file.read_exact(buf.as_mut_slice())?;
    let data = decode(buf).unwrap();
    Ok(Value::String(String::from_utf8(data).unwrap()))
}

fn read_color_struct(file: &mut dyn Reader) -> Result<Value> {
    let b = file.read_f32()?;
    let g = file.read_f32()?;
    let r = file.read_f32()?;
    let a = file.read_f32()?;
    Ok(Value::RGBA(r, g, b, a))
}

fn read_linear_color_struct(file: &mut dyn Reader) -> Result<Value> {
    Ok(Value::RGBA(
        file.read_f32()?,
        file.read_f32()?,
        file.read_f32()?,
        file.read_f32()?,
    ))
}

fn read_quat_struct(file: &mut dyn Reader) -> Result<Value> {
    Ok(Value::Quat(
        file.read_f32()?,
        file.read_f32()?,
        file.read_f32()?,
        file.read_f32()?,
    ))
}

fn read_net_struct(file: &mut dyn Reader) -> Result<Value> {
    file.read_i32()?;
    Ok(Value::String(file.read_str()?))
}

fn read_vector_struct(file: &mut dyn Reader) -> Result<Value> {
    Ok(Value::Vector(
        file.read_f32()?,
        file.read_f32()?,
        file.read_f32()?,
    ))
}

fn read_vector2d_struct(file: &mut dyn Reader) -> Result<Value> {
    Ok(Value::Vector2D(file.read_f32()?, file.read_f32()?))
}
