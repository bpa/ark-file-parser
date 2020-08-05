use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result, SeekFrom};

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

    println!("Version {}", read_i16(&mut file)?);
    let binary_data = read_i32(&mut file)?;
    file.seek(SeekFrom::Current(4))?;
    let name_table = read_i32(&mut file)?;

    println!("binary data at {:x}", binary_data);
    println!("name table at {:x}", name_table);

    file.seek(SeekFrom::Start(name_table as u64))?;
    let names = read_i32(&mut file)?;
    println!("Found {} names", names);
    for i in 1..10 {
        println!("{}: {}", i, read_string(&mut file)?);
    }
    Ok(())
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

fn read_string(f: &mut dyn Read) -> Result<String> {
    let size = read_i32(f)?;
    let mut buf = vec![0u8; size as usize];
    f.read_exact(&mut buf)?;
    match String::from_utf8(buf) {
        Ok(string) => Ok(string),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}
