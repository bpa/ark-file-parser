use crate::io::Name;
use std::collections::hash_map::{HashMap, Values};
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
pub struct Properties<'a> {
    pub props: HashMap<usize, Vec<Property>>,
    iter: Option<Values<'a, usize, Vec<Property>>>,
}

impl<'a> Properties<'a> {
    pub fn new() -> Self {
        Properties {
            props: HashMap::new(),
            iter: None,
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
