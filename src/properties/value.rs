use crate::io::Name;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Value {
    Array(Name),
    Bool(bool),
    Byte(u8),
    Double(f64),
    Enum(Name, Name),
    Float(f32),
    Int16(i16),
    Int8(i8),
    Int(i32),
    Name(Name),
    String(String),
    Struct(Name, u32),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Value::Array(ref name) => f.write_fmt(format_args!("{}", name.id)),
            Value::Bool(v) => f.write_fmt(format_args!("{}", v)),
            Value::Byte(v) => f.write_fmt(format_args!("{}", v)),
            Value::Double(v) => f.write_fmt(format_args!("{}", v)),
            Value::Enum(ref name, ref v) => f.write_fmt(format_args!("{}: {}", name.id, v.id)),
            Value::Float(v) => f.write_fmt(format_args!("{}", v)),
            Value::Int16(v) => f.write_fmt(format_args!("{}", v)),
            Value::Int8(v) => f.write_fmt(format_args!("{}", v)),
            Value::Int(v) => f.write_fmt(format_args!("{}", v)),
            Value::Name(ref name) => f.write_fmt(format_args!("{}", name.id)),
            Value::String(ref v) => f.write_str(v),
            Value::Struct(ref name, _v) => f.write_fmt(format_args!("{}", name.id)),
            Value::UInt16(v) => f.write_fmt(format_args!("{}", v)),
            Value::UInt32(v) => f.write_fmt(format_args!("{}", v)),
            Value::UInt64(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}
