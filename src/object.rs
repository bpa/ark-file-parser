use crate::io::Name;
use crate::location::Location;
use crate::names::Names;
use crate::properties::Properties;
use enumset::EnumSetType;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Object<'a> {
    pub guid: u128,
    pub name: Name,
    pub is_item: bool,
    pub extra_classes: Vec<Name>,
    pub location: Option<Location>,
    pub properties: Properties<'a>,
    pub object_type: Type,
}

impl<'a> Object<'a> {
    pub fn new(
        guid: u128,
        name: Name,
        is_item: bool,
        extra_classes: Vec<Name>,
        location: Option<Location>,
        properties: Properties<'a>,
        names: &Names,
    ) -> Self {
        lazy_static! {
            static ref EGG_RE: Regex = Regex::new(r"Egg.*Fertilized").unwrap();
        }

        let object_type = match 1 {
            _ if EGG_RE.is_match(&names[name.id]) => Type::FertilizedEgg,
            _ => Type::Unknown,
        };
        // DroppedItem,
        // Inventory,
        // Player,
        // Raft,
        // Structure,
        // TamedCreature,
        // Unknown,
        // WildCreature,

        Object {
            guid,
            name,
            is_item,
            extra_classes,
            location,
            properties,
            object_type,
        }
    }
}

#[derive(Debug, EnumSetType)]
pub enum Type {
    DroppedItem,
    FertilizedEgg,
    Inventory,
    Player,
    Raft,
    Status,
    Structure,
    TamedCreature,
    Unknown,
    WildCreature,
}
