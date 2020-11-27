use crate::io::Name;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
mod read;
mod value;
pub use value::Value;

#[derive(Debug)]
pub struct Property {
    pub name: Name,
    pub ind: u32,
    pub value: Value,
}

#[derive(Debug)]
pub struct Properties {
    pub props: HashMap<usize, Vec<Property>>,
    set: HashSet<String>,
}

impl<'a> Properties {
    pub fn new() -> Self {
        Properties {
            props: HashMap::new(),
            set: HashSet::new(),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.set.contains(name)
    }

    pub fn contains_id(&self, name: &usize) -> bool {
        self.props.contains_key(name)
    }

    pub fn get_bool(&self, name: &usize) -> Option<bool> {
        match self.props.get(name) {
            Some(value) => match value.get(0) {
                Some(prop) => match prop.value {
                    Value::Bool(value) => Some(value),
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
    }

    pub fn get_i32(&self, name: &usize) -> Option<i32> {
        match self.props.get(name) {
            Some(value) => match value.get(0) {
                Some(prop) => match prop.value {
                    Value::Byte(v) => Some(v as i32),
                    Value::Double(v) => Some(v as i32),
                    Value::Float(v) => Some(v as i32),
                    Value::Int16(v) => Some(v as i32),
                    Value::Int8(v) => Some(v as i32),
                    Value::Int(v) => Some(v),
                    Value::UInt16(v) => Some(v as i32),
                    Value::UInt32(v) => Some(v as i32),
                    Value::UInt64(v) => Some(v as i32),
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
    }

    pub fn get_vec_int(&self, name: &usize) -> Vec<i32> {
        match self.props.get(name) {
            Some(values) => {
                let mut list = Vec::new();
                let mut i = 0;
                for prop in values {
                    while i < prop.ind {
                        list.push(0);
                        i += 1;
                    }
                    list.push(match prop.value {
                        Value::Byte(v) => v as i32,
                        Value::Double(v) => v as i32,
                        Value::Float(v) => v as i32,
                        Value::Int16(v) => v as i32,
                        Value::Int8(v) => v as i32,
                        Value::Int(v) => v,
                        Value::UInt16(v) => v as i32,
                        Value::UInt32(v) => v as i32,
                        Value::UInt64(v) => v as i32,
                        _ => 0,
                    });
                    i += 1;
                }
                list
            }
            None => Vec::new(),
        }
    }

    pub fn get_str(&self, name: &usize) -> Option<&str> {
        match self.props.get(name) {
            Some(value) => match value.get(0) {
                Some(prop) => match &prop.value {
                    Value::String(value) => Some(value),
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
    }
}

// impl<'a> Iterator for Properties<'a> {
//     type Item = Vec<&'a Value>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.iter.is_none() {
//             self.iter = self.props.iter();
//         }
//         match self.iter {
//             Some(it) => match it.next() {
//                 Some(props) => Some(props.iter().map(|p| &p.value).collect()),
//                 None => {
//                     self.iter = None;
//                     None
//                 }
//             },
//             None => {
//                 self.iter = None;
//                 None
//             }
//         }
//     }
// }
