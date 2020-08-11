use crate::ark::Ark;
use crate::io::{MMappedReader, Reader};
use crate::properties::read_properties;
use std::io::{Result, Seek, SeekFrom};

pub struct ArkSave {
    file: MMappedReader,
    map: Ark,
    names: Vec<String>,
}

impl ArkSave {
    pub fn open(filename: &str) -> Result<Self> {
        Ok(ArkSave {
            file: MMappedReader::open(filename)?,
            map: Ark::new(50.0, 8000.0, 50.0, 8000.0),
            names: Vec::new(),
        })
    }

    pub fn read_all(&mut self) -> Result<()> {
        let (class_offset, properties_offset) = self.read_header()?;
        self.skip_binary_data_names()?;
        self.skip_embedded_binary_data()?;
        self.skip_unknown_data()?;

        self.read_class_names(class_offset)?;
        self.read_objects(properties_offset)?;
        Ok(())
    }

    fn read_header(&mut self) -> Result<(u64, u64)> {
        println!("Version {}", self.file.read_i16()?);
        self.file.seek(SeekFrom::Current(8))?;
        let classes_offset = self.file.read_i32()? as u64;
        let properties_offset = self.file.read_i32()? as u64;
        self.file.seek(SeekFrom::Current(8))?;
        Ok((classes_offset, properties_offset))
    }

    fn read_class(&mut self) -> Result<usize> {
        let name_ind = self.file.read_i32()? as usize;
        self.file.seek(SeekFrom::Current(4))?;
        Ok(name_ind)
    }

    fn read_location(&mut self) -> Result<(f32, f32, f32)> {
        let x = self.file.read_f32()?;
        let y = self.file.read_f32()?;
        let z = self.file.read_f32()?;
        self.file.seek(SeekFrom::Current(12))?;
        Ok((x, y, z))
    }

    fn skip_binary_data_names(&mut self) -> Result<()> {
        //TODO: Set map based on first binary name
        let count = self.file.read_i32()?;
        for _ in 0..count {
            let size = self.file.read_i32()?;
            self.file.seek(SeekFrom::Current(size as i64))?;
        }
        Ok(())
    }

    fn skip_embedded_binary_data(&mut self) -> Result<()> {
        let parts = self.file.read_i32()?;
        for _ in 0..parts {
            let blobs = self.file.read_i32()?;
            for _ in 0..blobs {
                let blob_size = self.file.read_i32()?;
                self.file.seek(SeekFrom::Current(blob_size as i64))?;
            }
        }
        Ok(())
    }

    fn skip_unknown_data(&mut self) -> Result<()> {
        let entries = self.file.read_i32()?;
        for _ in 0..entries {
            self.file.seek(SeekFrom::Current(8 as i64))?;
            let str_len = self.file.read_i32()? as i64;
            self.file.seek(SeekFrom::Current(str_len))?;
        }
        Ok(())
    }

    fn read_class_names(&mut self, class_offset: u64) -> Result<()> {
        let current_pos = self.file.seek(SeekFrom::Current(0))?;
        self.file.seek(SeekFrom::Start(class_offset))?;
        let name_count = self.file.read_i32()?;
        let mut names = Vec::with_capacity(name_count as usize + 1);
        //Name indexes start at 1, adding a dummy will align the indexes
        names.push(String::from("-----"));
        for _ in 0..name_count {
            names.push(self.file.read_str()?);
        }
        self.file.seek(SeekFrom::Start(current_pos))?;
        self.names.append(&mut names);
        Ok(())
    }

    fn read_objects(&mut self, properties_offset: u64) -> Result<()> {
        let objects = self.file.read_i32()?;
        println!("Found {} objects", objects);
        for _ in 0..objects {
            let here = self.file.seek(SeekFrom::Current(0))?;
            let guid = self.file.read_u128()?;
            let class = self.read_class()?;
            let _is_item = self.file.read_bool()?;
            let extra_classes = self.file.read_i32()?;
            let mut classes = Vec::with_capacity(extra_classes as usize);
            for _ in 0..extra_classes {
                classes.push(self.read_class()?);
            }
            self.file.seek(SeekFrom::Current(8))?;
            let skip = self.names[class] == "InstancedFoliageActor";
            if self.file.read_bool()? {
                //has location
                let (x, y, z) = self.read_location()?;
                if !skip {
                    println!(
                        "^^^\nFound {} ({:x}) at {}, {}, {} ({:.1}, {:.1})",
                        self.names[class],
                        guid,
                        x,
                        y,
                        z,
                        (x / self.map.x_divisor) + self.map.x_offset,
                        (y / self.map.y_divisor) + self.map.y_offset
                    );
                }
            } else {
                println!("vvv\nFound {} ({:x})", self.names[class], guid);
            }
            if !skip {
                if class != 0 {
                    read_properties(&mut self.file, &mut self.names, properties_offset)?;
                }
            } else {
                self.file.read_i32()?;
            }
            self.file.seek(SeekFrom::Current(4))?;
        }
        Ok(())
    }
}
