use crate::io::Reader;
// use std::collections::hash_map::HashMap;
use std::io::{Result, SeekFrom};
use std::ops::Index;

pub struct Names {
    names: Vec<String>,
    // lookup: HashMap<&'a str, usize>,
}

impl Names {
    pub fn new(file: &mut dyn Reader, class_offset: u64) -> Result<Self> {
        let current_pos = file.seek(SeekFrom::Current(0))?;
        file.seek(SeekFrom::Start(class_offset))?;
        let name_count = file.read_i32()?;
        let capacity = name_count as usize + 1;
        let mut names = Vec::with_capacity(capacity);
        // let mut lookup = HashMap::with_capacity(capacity);
        //Name indexes start at 1, adding a dummy will align the indexes
        names.push(String::from("-----"));
        for _i in 1..=name_count {
            let name = file.read_str()?;
            // lookup.insert(name.as_str(), i as usize);
            names.push(name);
        }
        file.seek(SeekFrom::Start(current_pos))?;
        Ok(Names { names })
    }
}

impl<'a> Index<usize> for Names {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        self.names[i].as_str()
    }
}
