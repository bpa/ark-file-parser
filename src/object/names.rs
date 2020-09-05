use crate::io::Reader;
use std::collections::hash_map::HashMap;
use std::io::{Result, SeekFrom};
use std::ops::Index;
use std::rc::Rc;

pub struct Names {
    names: Vec<Rc<String>>,
    lookup: HashMap<Rc<String>, usize>,
}

impl Names {
    pub fn new(file: &mut dyn Reader, class_offset: u64) -> Result<Self> {
        let current_pos = file.seek(SeekFrom::Current(0))?;
        file.seek(SeekFrom::Start(class_offset))?;
        let name_count = file.read_i32()?;
        let capacity = name_count as usize + 1;
        let mut names = Vec::with_capacity(capacity);
        let mut lookup = HashMap::with_capacity(capacity);
        // Name indexes start at 1, adding a dummy will align the indexes
        names.push(Rc::new(String::from("-----")));
        for i in 1..=name_count {
            let name = Rc::new(file.read_str()?);
            names.push(name.clone());
            lookup.insert(name, i as usize);
        }
        file.seek(SeekFrom::Start(current_pos))?;
        Ok(Names { names, lookup })
    }

    pub fn get_name_id(&self, name: &str) -> Option<&usize> {
        self.lookup.get(&Rc::new(String::from(name)))
    }
}

impl Index<usize> for Names {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        self.names[i].as_str()
    }
}
