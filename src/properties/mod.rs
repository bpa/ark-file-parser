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
