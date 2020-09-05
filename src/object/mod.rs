use self::location::Location;
pub use self::names::Names;
pub use self::savefile::ArkSave;
use crate::io::Name;
use crate::properties::Properties;
use enumset::EnumSetType;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::rc::Rc;
mod location;
mod names;
mod savefile;
mod serialize;

pub struct Object {
    pub guid: u128,
    pub name: Name,
    pub is_item: bool,
    pub extra_classes: Vec<Name>,
    pub location: Option<Location>,
    pub properties: Properties,
    pub object_type: Type,
    names: Rc<Names>,
}

impl Object {
    pub fn new(
        guid: u128,
        name: Name,
        is_item: bool,
        extra_classes: Vec<Name>,
        location: Option<Location>,
        properties: Properties,
        names: Rc<Names>,
    ) -> Self {
        lazy_static! {
            static ref EGG_RE: Regex = Regex::new(r"Egg.*Fertilized").unwrap();
        }

        let class = &names[name.id];
        let object_type = match 1 {
            _ if is_item => Type::Item,
            _ if properties.contains("OwnerName") || properties.contains("bHasResetDecayTime") => {
                if class.starts_with("DeathItemCache_") {
                    Type::DeathItemCache
                } else {
                    Type::Structure
                }
            }
            _ if class.starts_with("DinoTamedInventoryComponent_") => Type::TamedInventory,
            _ if properties.contains("bInitializedMe") => {
                if class.starts_with("PrimalInventoryBP_") {
                    Type::StructureInventory
                } else if class.starts_with("PrimalInventoryComponent") {
                    Type::PlayerInventory
                } else if class.starts_with("DinoWildInventoryComponent_") {
                    Type::WildCreatureInventory
                } else {
                    Type::Unknown
                }
            }
            _ if class == "Raft_BP_C" || class == "MotorRaft_BP_C" => Type::Raft,
            _ if properties.contains("DinoID1") => {
                if properties.contains("TamerString") || properties.contains("TamingTeamID") {
                    Type::TamedCreature
                } else {
                    Type::WildCreature
                }
            }
            _ if properties.contains("CurrentStatusValues") => Type::StatusValues,
            _ if class == "StructurePaintingComponent" => Type::StructurePaintingComponent,
            _ if class.starts_with("DroppedItem") => Type::DroppedItem,
            _ if EGG_RE.is_match(&class) => Type::FertilizedEgg,
            _ if class == "PlayerPawnTest_Male_C" || class == "PlayerPawnTest_Female_C" => {
                Type::Player
            }
            _ if class.starts_with("BossArenaManager")
                || class == "ShooterGameState"
                || class == "TestGameMode_C"
                || class.starts_with("NPCZoneManager")
                || class.starts_with("WeapFists")
                || class.ends_with("Manager")
                || class.ends_with("Actor") =>
            {
                Type::Game
            }
            _ => Type::Unknown,
        };

        Object {
            guid,
            name,
            is_item,
            extra_classes,
            location,
            properties,
            object_type,
            names,
        }
    }
}

#[derive(Debug, EnumSetType, Serialize)]
pub enum Type {
    DeathItemCache,
    DroppedItem,
    FertilizedEgg,
    Game,
    Item,
    Player,
    PlayerInventory,
    Raft,
    StatusValues,
    Structure,
    StructureInventory,
    StructurePaintingComponent,
    TamedCreature,
    TamedInventory,
    Unknown,
    WildCreature,
    WildCreatureInventory,
}
