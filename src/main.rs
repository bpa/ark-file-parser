use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result, SeekFrom};

const X_OFFSET: f32 = 50.0;
const X_DIVISOR: f32 = 8000.0;
const Y_OFFSET: f32 = 50.0;
const Y_DIVISOR: f32 = 8000.0;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        read_file(&args[1])?;
    } else {
        println!("Usage: {} <savefile>", args[0]);
    }
    Ok(())
}

fn read_file(filename: &String) -> Result<()> {
    let mut file = File::open(filename)?;

    let (name_table, properties) = read_header(&mut file)?;

    skip_binary_data_names(&mut file)?;
    skip_embedded_binary_data(&mut file)?;
    skip_unknown_data(&mut file)?;

    let class_names = read_class_names(&mut file, name_table)?;
    read_objects(&mut file, &class_names, properties)?;

    Ok(())
}

fn read_header(file: &mut File) -> Result<(u64, u64)> {
    println!("Version {}", read_i16(file)?);
    file.seek(SeekFrom::Current(8))?;
    let name_table = read_i32(file)? as u64;
    let properties = read_i32(file)? as u64;
    file.seek(SeekFrom::Current(8))?;
    Ok((name_table, properties))
}

fn read_i16(f: &mut dyn Read) -> Result<i16> {
    let mut buf = [0; 2];
    f.read(&mut buf)?;
    Ok(i16::from_le_bytes(buf))
}

fn read_i32(f: &mut dyn Read) -> Result<i32> {
    let mut buf = [0; 4];
    f.read(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

fn read_f32(f: &mut dyn Read) -> Result<f32> {
    let mut buf = [0; 4];
    f.read(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read_string(f: &mut dyn Read) -> Result<String> {
    let size = read_i32(f)?;
    let mut buf = vec![0u8; size as usize];
    f.read_exact(&mut buf)?;
    match String::from_utf8(buf) {
        Ok(string) => Ok(string),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}

fn read_boolean(f: &mut dyn Read) -> Result<bool> {
    let mut buf = [0; 4];
    f.read(&mut buf)?;
    Ok(i32::from_le_bytes(buf) == 1)
}

fn read_class<'a>(file: &mut File, names: &'a Vec<String>) -> Result<&'a String> {
    let name_ind = read_i32(file)? as usize;
    file.seek(SeekFrom::Current(4))?;
    Ok(&names[name_ind])
}

fn read_location(file: &mut File) -> Result<(f32, f32, f32)> {
    let x = read_f32(file)?;
    let y = read_f32(file)?;
    let z = read_f32(file)?;
    file.seek(SeekFrom::Current(12))?;
    Ok((x, y, z))
}

fn read_properties(f: &mut File, _offset: u64) -> Result<()> {
    f.seek(SeekFrom::Current(4))?;
    Ok(())
}

fn skip_binary_data_names(f: &mut File) -> Result<()> {
    let count = read_i32(f)?;
    for _ in 0..count {
        let size = read_i32(f)?;
        f.seek(SeekFrom::Current(size as i64))?;
    }
    Ok(())
}

fn skip_embedded_binary_data(f: &mut File) -> Result<()> {
    let parts = read_i32(f)?;
    for _ in 0..parts {
        let blobs = read_i32(f)?;
        for _ in 0..blobs {
            let blob_size = read_i32(f)?;
            f.seek(SeekFrom::Current(blob_size as i64))?;
        }
    }
    Ok(())
}

fn skip_unknown_data(f: &mut File) -> Result<()> {
    let entries = read_i32(f)?;
    for _ in 0..entries {
        f.seek(SeekFrom::Current(8 as i64))?;
        let str_len = read_i32(f)? as i64;
        f.seek(SeekFrom::Current(str_len))?;
    }
    Ok(())
}

fn read_class_names(file: &mut File, offset: u64) -> Result<Vec<String>> {
    let current_pos = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(offset as u64))?;
    let name_count = read_i32(file)?;
    let mut names = Vec::with_capacity(name_count as usize + 1);
    //Names indexes start at 1, adding a dummy will align the indexes
    names.push(String::from("Zero"));
    for _ in 0..name_count {
        names.push(read_string(file)?);
    }
    file.seek(SeekFrom::Start(current_pos))?;
    Ok(names)
}

fn read_objects(file: &mut File, names: &Vec<String>, properties: u64) -> Result<()> {
    let objects = read_i32(file)?;
    println!("Found {} objects", objects);
    for _ in 0..objects {
        file.seek(SeekFrom::Current(16))?; //Skip GUID
        let class = read_class(file, names)?;
        let _is_item = read_boolean(file)?;
        let extra_classes = read_i32(file)?;
        let mut classes = Vec::with_capacity(extra_classes as usize);
        for _ in 0..extra_classes {
            classes.push(read_class(file, names)?);
        }
        file.seek(SeekFrom::Current(8))?;
        if read_boolean(file)? {
            //has location
            let (x, y, z) = read_location(file)?;
            if class.contains("Unicorn") {
                println!(
                    "Found Unicorn at {}, {}, {} ({:.1}, {:.1})",
                    x,
                    y,
                    z,
                    (x / X_DIVISOR) + X_OFFSET,
                    (y / Y_DIVISOR) + Y_OFFSET
                );
            }
        }
        read_properties(file, properties)?;
        file.seek(SeekFrom::Current(4))?;
    }
    Ok(())
}
