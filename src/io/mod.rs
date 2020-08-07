mod mmap;

pub use mmap::MMappedReader;
use std::io::{Read, Result, Seek};

pub trait Reader: Seek + Read {
    fn read_bool(&mut self) -> Result<bool>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_i8(&mut self) -> Result<i8>;
    fn read_str(&mut self) -> Result<String>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_u8(&mut self) -> Result<u8>;
    //0008 ArrayProperty
    //0009 StructProperty
    //0015 ObjectProperty
    //0084 NameProperty
}
