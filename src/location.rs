use crate::io::Reader;
use std::io::{Result, SeekFrom};

#[derive(Debug)]
pub struct Location {
    x: f32,
    y: f32,
    z: f32,
}

impl Location {
    pub fn read(file: &mut dyn Reader) -> Result<Self> {
        let x = file.read_f32()?;
        let y = file.read_f32()?;
        let z = file.read_f32()?;
        file.seek(SeekFrom::Current(12))?;
        Ok(Location { x, y, z })
    }
}
