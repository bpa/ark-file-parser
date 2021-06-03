use crate::io::Reader;
use memmap::Mmap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::io::{Seek, SeekFrom};

pub struct MMappedReader {
    file: File,
    mmap: Mmap,
    offset: usize,
}

impl MMappedReader {
    pub fn open(filename: &str) -> Result<Self> {
        let file = File::open(filename)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(MMappedReader {
            file,
            mmap,
            offset: 0,
        })
    }
}

impl Read for MMappedReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = buf.len();
        buf.copy_from_slice(&self.mmap[self.offset..self.offset + bytes]);
        self.offset += bytes;
        Ok(bytes)
    }
}

impl Seek for MMappedReader {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.offset = match pos {
            SeekFrom::Current(ind) => self.offset + ind as usize,
            SeekFrom::End(ind) => (self.file.metadata()?.len() as i64 - ind) as usize,
            SeekFrom::Start(ind) => ind as usize,
        };
        Ok(self.offset as u64)
    }
}

impl Reader for MMappedReader {
    fn read_bool(&mut self) -> Result<bool> {
        let buf = array_ref![self.mmap, self.offset, 4];
        self.offset += 4;
        Ok(i32::from_le_bytes(*buf) == 1)
    }

    fn read_f32(&mut self) -> Result<f32> {
        let buf = array_ref![self.mmap, self.offset, 4];
        self.offset += 4;
        Ok(f32::from_le_bytes(*buf))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let buf = array_ref![self.mmap, self.offset, 8];
        self.offset += 8;
        Ok(f64::from_le_bytes(*buf))
    }

    fn read_i16(&mut self) -> Result<i16> {
        let buf = array_ref![self.mmap, self.offset, 2];
        self.offset += 2;
        Ok(i16::from_le_bytes(*buf))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let buf = array_ref![self.mmap, self.offset, 4];
        self.offset += 4;
        Ok(i32::from_le_bytes(*buf))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let buf = array_ref![self.mmap, self.offset, 8];
        self.offset += 8;
        Ok(i64::from_le_bytes(*buf))
    }

    fn read_i8(&mut self) -> Result<i8> {
        let val = self.mmap[self.offset];
        self.offset += 1;
        Ok(val as i8)
    }

    fn read_str(&mut self) -> Result<String> {
        let size = self.read_i32()?;
        match size {
            0 | 1 => {
                self.offset += size as usize;
                return Ok(String::from(""));
            }
            -1 => {
                self.offset += 2;
                return Ok(String::from(""));
            }
            _ if size < 0 => {
                let size = (size * -2) as usize;
                let mut buf = Vec::with_capacity(size - 1);
                buf.extend_from_slice(&self.mmap[self.offset..self.offset + size - 1]);
                let data: &[u16] =
                    unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u16, buf.len()) };
                self.offset += size;
                match String::from_utf16(data) {
                    Ok(string) => Ok(string),
                    Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
                }
            }
            _ => {
                let size = size as usize;
                let mut buf = Vec::with_capacity(size - 1);
                buf.extend_from_slice(&self.mmap[self.offset..self.offset + size - 1]);
                self.offset += size;
                match String::from_utf8(buf) {
                    Ok(string) => Ok(string),
                    Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
                }
            }
        }
    }

    fn read_u128(&mut self) -> Result<u128> {
        let buf = array_ref![self.mmap, self.offset, 16];
        self.offset += 16;
        Ok(u128::from_le_bytes(*buf))
    }

    fn read_u16(&mut self) -> Result<u16> {
        let buf = array_ref![self.mmap, self.offset, 2];
        self.offset += 2;
        Ok(u16::from_le_bytes(*buf))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let buf = array_ref![self.mmap, self.offset, 4];
        self.offset += 4;
        Ok(u32::from_le_bytes(*buf))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let buf = array_ref![self.mmap, self.offset, 8];
        self.offset += 8;
        Ok(u64::from_le_bytes(*buf))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let val = self.mmap[self.offset];
        self.offset += 1;
        Ok(val)
    }

    fn skip_str(&mut self) -> Result<()> {
        let size = self.read_i32()?;
        self.offset += if size < 0 {
            (size * -2) as usize
        } else {
            size as usize
        };

        Ok(())
    }
}
