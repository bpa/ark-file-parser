mod location;
mod names;
mod object;
mod savefile;
mod serialize;
use super::properties::Properties;
pub use crate::io::Name;
pub use location::Location;
pub use names::Names;
pub use object::{Object, Type};
pub use savefile::ArkSave;
use std::rc::Rc;

pub struct Entry {
    pub object_type: Type,
    objects: Rc<Vec<Object>>,
    object: usize,
    inventory: Option<usize>,
    status: Option<usize>,
}

impl Entry {
    pub fn inventory_component(&self) -> Option<&Object> {
        match self.inventory {
            Some(index) => Some(&self.objects[index]),
            None => None,
        }
    }

    pub fn location(&self) -> Option<&Location> {
        match &self.objects[self.object].location {
            Some(loc) => Some(loc),
            None => None,
        }
    }

    pub fn name(&self) -> &Name {
        &self.objects[self.object].name
    }

    pub fn properties(&self) -> &Properties {
        &self.objects[self.object].properties
    }

    pub fn status_component(&self) -> Option<&Object> {
        match self.status {
            Some(index) => Some(&self.objects[index]),
            None => None,
        }
    }
}
