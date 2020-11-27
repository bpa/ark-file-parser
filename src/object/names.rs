use crate::io::Reader;
use std::collections::hash_map::HashMap;
use std::io::{Result, SeekFrom};
use std::ops::Index;
use std::rc::Rc;

pub struct Names {
    names: Vec<Rc<String>>,
    lookup: HashMap<Rc<String>, usize>,
    pub array_property: usize,
    pub bool_property: usize,
    pub byte_property: usize,
    pub color_property: usize,
    pub double_property: usize,
    pub float_property: usize,
    pub int16_property: usize,
    pub int8_property: usize,
    pub int_property: usize,
    pub linear_color_property: usize,
    pub name_property: usize,
    pub object_property: usize,
    pub quat_property: usize,
    pub rotator_property: usize,
    pub str_property: usize,
    pub struct_property: usize,
    pub text_property: usize,
    pub uint16_property: usize,
    pub uint32_property: usize,
    pub uint64_property: usize,
    pub unique_netid_property: usize,
    pub vector_property: usize,
    pub vector2d_property: usize,
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

        let array_property = name_id(&lookup, "ArrayProperty");
        let bool_property = name_id(&lookup, "BoolProperty");
        let byte_property = name_id(&lookup, "ByteProperty");
        let color_property = name_id(&lookup, "Color");
        let double_property = name_id(&lookup, "DoubleProperty");
        let float_property = name_id(&lookup, "FloatProperty");
        let int16_property = name_id(&lookup, "Int16Property");
        let int8_property = name_id(&lookup, "Int8Property");
        let int_property = name_id(&lookup, "IntProperty");
        let linear_color_property = name_id(&lookup, "LinearColor");
        let name_property = name_id(&lookup, "NameProperty");
        let object_property = name_id(&lookup, "ObjectProperty");
        let quat_property = name_id(&lookup, "Quat");
        let rotator_property = name_id(&lookup, "Rotator");
        let str_property = name_id(&lookup, "StrProperty");
        let struct_property = name_id(&lookup, "StructProperty");
        let text_property = name_id(&lookup, "TextProperty");
        let uint16_property = name_id(&lookup, "UInt16Property");
        let uint32_property = name_id(&lookup, "UInt32Property");
        let uint64_property = name_id(&lookup, "UInt64Property");
        let unique_netid_property = name_id(&lookup, "UniqueNetIdRepl");
        let vector_property = name_id(&lookup, "Vector");
        let vector2d_property = name_id(&lookup, "Vector2D");

        Ok(Names {
            names,
            lookup,
            array_property,
            bool_property,
            byte_property,
            color_property,
            double_property,
            float_property,
            int16_property,
            int8_property,
            int_property,
            linear_color_property,
            name_property,
            object_property,
            quat_property,
            rotator_property,
            str_property,
            struct_property,
            text_property,
            uint16_property,
            uint32_property,
            uint64_property,
            unique_netid_property,
            vector_property,
            vector2d_property,
        })
    }

    pub fn get_name_id(&self, name: &str) -> Option<&usize> {
        self.lookup.get(&Rc::new(String::from(name)))
    }
}

fn name_id(lookup: &HashMap<Rc<String>, usize>, name: &str) -> usize {
    *lookup.get(&Rc::new(String::from(name))).unwrap_or(&0)
}

impl Names {
    pub fn len(&self) -> usize {
        self.names.len()
    }
}

impl Index<usize> for Names {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        self.names[i].as_str()
    }
}
