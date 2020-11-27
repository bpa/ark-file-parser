use super::{Entry, Names, Object};
use crate::properties::{Properties, Property, Value};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeTuple, Serializer};
use std::rc::Rc;

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.objects[self.object].serialize(serializer)
    }
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("ClassName", &self.names[self.name.id])?;
        map.serialize_entry("Classification", &self.object_type)?;
        if let Some(status) = self.status_component {
            map.serialize_entry("StatusComponent", &status)?;
        }
        if let Some(inventory) = self.inventory_component {
            map.serialize_entry("InventoryComponent", &inventory)?;
        }
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

struct NameProperty<'a> {
    names: &'a Rc<Names>,
    property: &'a Vec<Property>,
}

macro_rules! serialize {
    ($serializer:ident, $value:ident) => {{
        let mut seq = $serializer.serialize_seq(Some($value.len()))?;
        for element in $value {
            seq.serialize_element(element)?;
        }
        seq.end()
    }};
}

impl<'a> Serialize for NameValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value {
            Value::ArrayOfF32(v) => serialize!(serializer, v),
            Value::ArrayOfF64(v) => serialize!(serializer, v),
            Value::ArrayOfI16(v) => serialize!(serializer, v),
            Value::ArrayOfI32(v) => serialize!(serializer, v),
            Value::ArrayOfI8(v) => serialize!(serializer, v),
            Value::ArrayOfName(v) => serialize!(serializer, v),
            Value::ArrayOfObject(v) => serialize!(serializer, v),
            Value::ArrayOfStruct(v) => serialize!(serializer, v),
            Value::ArrayOfStr(v) => serialize!(serializer, v),
            Value::ArrayOfU16(v) => serialize!(serializer, v),
            Value::ArrayOfU32(v) => serialize!(serializer, v),
            Value::ArrayOfU64(v) => serialize!(serializer, v),
            Value::ArrayOfU8(v) => serialize!(serializer, v),
            Value::ArrayOfBool(v) => serialize!(serializer, v),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Byte(v) => serializer.serialize_bytes(&[*v]),
            Value::Double(v) => serializer.serialize_f64(*v),
            Value::Enum(v, _) => serializer.serialize_str(&self.names[v.id]),
            Value::Float(v) => serializer.serialize_f32(*v),
            Value::Int16(v) => serializer.serialize_i16(*v),
            Value::Int8(v) => serializer.serialize_i8(*v),
            Value::Int(v) => serializer.serialize_i32(*v),
            Value::Name(v) => serializer.serialize_str(&self.names[v.id]),
            Value::Properties(v) => {
                let mut map = serializer.serialize_map(Some(v.props.len()))?;
                for (k, p) in &v.props {
                    map.serialize_entry(
                        &self.names[*k],
                        &NameProperty {
                            names: self.names,
                            property: p,
                        },
                    )?;
                }
                map.end()
            }
            Value::Quat(x, y, z, w) => {
                let mut tup = serializer.serialize_tuple(4)?;
                tup.serialize_element(&x)?;
                tup.serialize_element(&y)?;
                tup.serialize_element(&z)?;
                tup.serialize_element(&w)?;
                tup.end()
            }
            Value::RGBA(r, g, b, a) => {
                let mut tup = serializer.serialize_tuple(4)?;
                tup.serialize_element(&r)?;
                tup.serialize_element(&g)?;
                tup.serialize_element(&b)?;
                tup.serialize_element(&a)?;
                tup.end()
            }
            Value::String(v) => serializer.serialize_str(v.as_str()),
            Value::UInt16(v) => serializer.serialize_u16(*v),
            Value::UInt32(v) => serializer.serialize_u32(*v),
            Value::UInt64(v) => serializer.serialize_u64(*v),
            Value::Vector(x, y, z) => {
                let mut tup = serializer.serialize_tuple(3)?;
                tup.serialize_element(&x)?;
                tup.serialize_element(&y)?;
                tup.serialize_element(&z)?;
                tup.end()
            }
            Value::Vector2D(x, y) => {
                let mut tup = serializer.serialize_tuple(2)?;
                tup.serialize_element(&x)?;
                tup.serialize_element(&y)?;
                tup.end()
            }
        }
    }
}

impl<'a> Serialize for NameProperty<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.property.len() == 1 {
            NameValue {
                names: self.names,
                value: &self.property[0].value,
            }
            .serialize(serializer)
        } else {
            let mut seq = serializer.serialize_seq(Some(self.property.len()))?;
            for p in self.property {
                seq.serialize_element(&NameValue {
                    names: self.names,
                    value: &p.value,
                })?;
            }
            seq.end()
        }
    }
}

impl Serialize for Properties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = serializer.serialize_map(Some(self.props.len()))?;
        map.end()
    }
}
