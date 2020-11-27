use crate::io::Name;
use serde::Serialize;
use std::fmt::{Display, Formatter};

use super::Properties;

#[derive(Debug, Serialize)]
pub enum Value {
    ArrayOfF32(Vec<f32>),
    ArrayOfF64(Vec<f64>),
    ArrayOfI16(Vec<i16>),
    ArrayOfI32(Vec<i32>),
    ArrayOfI8(Vec<i8>),
    ArrayOfName(Vec<Name>),
    ArrayOfObject(Vec<Value>),
    ArrayOfStruct(Vec<Value>),
    ArrayOfStr(Vec<String>),
    ArrayOfU16(Vec<u16>),
    ArrayOfU32(Vec<u32>),
    ArrayOfU64(Vec<u64>),
    ArrayOfU8(Vec<u8>),
    ArrayOfBool(Vec<bool>),
    Bool(bool),
    Byte(u8),
    Double(f64),
    Enum(Name, Name),
    Float(f32),
    Int16(i16),
    Int8(i8),
    Int(i32),
    Name(Name),
    Properties(Properties),
    Quat(f32, f32, f32, f32),
    RGBA(f32, f32, f32, f32),
    String(String),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Vector(f32, f32, f32),
    Vector2D(f32, f32),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Value::ArrayOfF32(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfF64(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfI16(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfI32(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfI8(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfName(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfObject(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfStruct(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfStr(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfU16(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfU32(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfU64(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfU8(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::ArrayOfBool(v) => f.write_fmt(format_args!("{:?}", v)),
            Value::Bool(v) => f.write_fmt(format_args!("{}", v)),
            Value::Byte(v) => f.write_fmt(format_args!("{}", v)),
            Value::Double(v) => f.write_fmt(format_args!("{}", v)),
            Value::Enum(ref name, ref v) => f.write_fmt(format_args!("{}: {}", name.id, v.id)),
            Value::Float(v) => f.write_fmt(format_args!("{}", v)),
            Value::Int16(v) => f.write_fmt(format_args!("{}", v)),
            Value::Int8(v) => f.write_fmt(format_args!("{}", v)),
            Value::Int(v) => f.write_fmt(format_args!("{}", v)),
            Value::Name(ref name) => f.write_fmt(format_args!("{}", name.id)),
            Value::Properties(p) => {
                f.write_str("{\n")?;
                for (name, props) in &p.props {
                    f.write_fmt(format_args!("\t{:>4}: {:?}", name, props))?;
                }
                f.write_str("\n}")
            }
            Value::Quat(x, y, z, w) => f.write_fmt(format_args!("({},{},{},{})", x, y, z, w)),
            Value::RGBA(r, g, b, a) => f.write_fmt(format_args!(
                "#{:02X}{:02X}{:02X}{:02X}",
                (r * 255f32) as isize,
                (g * 255f32) as isize,
                (b * 255f32) as isize,
                (a * 255f32) as isize,
            )),
            Value::String(ref v) => f.write_str(v),
            Value::UInt16(v) => f.write_fmt(format_args!("{}", v)),
            Value::UInt32(v) => f.write_fmt(format_args!("{}", v)),
            Value::UInt64(v) => f.write_fmt(format_args!("{}", v)),
            Value::Vector(x, y, z) => f.write_fmt(format_args!("({},{},{})", x, y, z)),
            Value::Vector2D(x, y) => f.write_fmt(format_args!("({},{})", x, y)),
        }
    }
}
