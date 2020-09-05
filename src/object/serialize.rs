use crate::object::{Names, Object};
use crate::properties::{Property, Value};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::rc::Rc;

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("ClassName", &self.names[self.name.id])?;
        map.serialize_entry("Classification", &self.object_type)?;
        for p in self.properties.props.values() {
            map.serialize_key(&self.names[p[0].name.id])?;
            if p.len() == 1 {
                map.serialize_value(&NameValue {
                    names: &self.names,
                    value: &p[0].value,
                })?;
            } else {
                map.serialize_value(&ValueVec {
                    names: &self.names,
                    values: p,
                })?;
            }
        }
        map.end()
    }
}

struct ValueVec<'a> {
    names: &'a Rc<Names>,
    values: &'a Vec<Property>,
}
impl<'a> Serialize for ValueVec<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut list = serializer.serialize_seq(Some(self.values.len()))?;
        let mut i = 0;
        for v in self.values {
            while i < v.ind {
                list.serialize_element(&0)?;
                i += 1;
            }
            list.serialize_element(&NameValue {
                names: &self.names,
                value: &v.value,
            })?;
            i += 1;
        }
        list.end()
    }
}

struct NameValue<'a> {
    names: &'a Rc<Names>,
    value: &'a Value,
}

impl<'a> Serialize for NameValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value {
            Value::Array(v) => serializer.serialize_str(&self.names[v.id]),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Byte(v) => serializer.serialize_bytes(&[*v]),
            Value::Double(v) => serializer.serialize_f64(*v),
            Value::Enum(v, _) => serializer.serialize_str(&self.names[v.id]),
            Value::Float(v) => serializer.serialize_f32(*v),
            Value::Int16(v) => serializer.serialize_i16(*v),
            Value::Int8(v) => serializer.serialize_i8(*v),
            Value::Int(v) => serializer.serialize_i32(*v),
            Value::Name(v) => serializer.serialize_str(&self.names[v.id]),
            Value::String(v) => serializer.serialize_str(v.as_str()),
            Value::Struct(v, _) => serializer.serialize_str(&self.names[v.id]),
            Value::UInt16(v) => serializer.serialize_u16(*v),
            Value::UInt32(v) => serializer.serialize_u32(*v),
            Value::UInt64(v) => serializer.serialize_u64(*v),
        }
    }
}
