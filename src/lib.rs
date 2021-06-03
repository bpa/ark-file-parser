mod file;
mod io;
mod object;
mod properties;

#[macro_use]
extern crate arrayref;
extern crate base64;
extern crate lazy_static;
extern crate paste;
extern crate serde;

pub use file::ArkParser;
pub use io::MMappedReader;
pub use object::{Entry, Location, Object, Type};
