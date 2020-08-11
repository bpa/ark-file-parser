use crate::io::Reader;
use std::io::{Result, SeekFrom};

pub enum Property {
    ArrayProperty,
    BoolProperty(bool),
    ByteProperty(u8),
    DoubleProperty(f64),
    EnumProperty(u8),
    FloatProperty(f32),
    Int16Property(i16),
    Int8Property(i8),
    IntProperty(i32),
    NameProperty,
    ObjectProperty,
    StrProperty(String),
    StructProperty,
    TextProperty,
    UInt16Property(u16),
    UInt32Property(u32),
    UInt64Property(u64),
}

pub fn read_properties(
    file: &mut dyn Reader,
    names: &Vec<String>,
    properties_offset: u64,
) -> Result<()> {
    let p = file.read_i32()? as u64;
    let here = file.seek(SeekFrom::Current(0))?;
    println!("Properties: {}", p + properties_offset);
    file.seek(SeekFrom::Start(p + properties_offset))?;
    for _ in 0..16 {
        let class_id = file.read_u32()? as usize;
        if names[class_id] == "None" {
            break;
        }
        let instance = file.read_u32()?;
        let var_type = file.read_u32()? as usize;
        file.seek(SeekFrom::Current(4))?;
        let data_size = file.read_u32()?;
        let ind = file.read_u32()?;
        print!(
            "{} {} {} {} {} ",
            names[class_id], instance, names[var_type], data_size, ind
        );

        match names[var_type].as_str() {
            "ArrayProperty" => read_array_property(file, names, data_size)?,
            "BoolProperty" => println!("{}", file.read_u8()? != 0),
            "ByteProperty" => read_byte_property(file, names)?,
            "DoubleProperty" => println!("{}", file.read_f64()?),
            "FloatProperty" => println!("{}", file.read_f32()?),
            "Int16Property" => println!("{}", file.read_i16()?),
            "Int8Property" => println!("{}", file.read_i8()?),
            "IntProperty" => println!("{}", file.read_i32()?),
            "NameProperty" => {
                let name = file.read_name()?;
                println!("{}({})", names[name.id], name.instance);
            }
            "ObjectProperty" => read_object_property(file, names, data_size)?,
            "StrProperty" => println!("{}", file.read_str()?),
            "StructProperty" => read_struct_property(file, data_size)?,
            "TextProperty" => panic!("Implement text"),
            "UInt16Property" => println!("{}", file.read_u16()?),
            "UInt32Property" => println!("{}", file.read_u32()?),
            "UInt64Property" => println!("{}", file.read_u64()?),
            _ => panic!("Unknown type {}", names[var_type]),
        }
    }
    file.seek(SeekFrom::Start(here))?;
    Ok(())
}

fn read_object_property(file: &mut dyn Reader, names: &Vec<String>, data_size: u32) -> Result<()> {
    match data_size {
        4 => println!("Obj: {}", file.read_i32()?),
        x if x >= 8 => {
            let prop_type = file.read_u32()?;
            if prop_type == 0 {
                // ID
                let id = file.read_i32()?;
                println!("ID: {}", id);
            } else if prop_type == 1 {
                // Path
                let name = file.read_name()?;
                println!("Path({}): {}", name.instance, names[name.id]);
            }
        }
        _ => {
            file.read_i32()?;
            println!("???");
        }
    }
    Ok(())
}

fn read_struct_property(file: &mut dyn Reader, data_size: u32) -> Result<()> {
    let _name = file.read_name()?;
    let pos = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(pos + data_size as u64))?;
    Ok(())
}

fn read_array_property(file: &mut dyn Reader, names: &Vec<String>, data_size: u32) -> Result<()> {
    let array_type = file.read_name()?;
    file.seek(SeekFrom::Current(data_size as i64))?;
    println!("Array");
    Ok(())
}

fn read_byte_property(file: &mut dyn Reader, names: &Vec<String>) -> Result<()> {
    let enum_name = file.read_name()?;
    if names[enum_name.id] == "None" {
        let value = file.read_u8()?;
        println!("{}({}) {}", names[enum_name.id], enum_name.instance, value);
    } else {
        //Its an enum, not a byte
        let value = file.read_name()?;
        println!(
            "{}({}) {}({})",
            names[enum_name.id], enum_name.instance, names[value.id], value.instance
        );
    }
    Ok(())
}
