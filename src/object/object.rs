use super::{Location, Names};
use crate::io::Name;
use crate::properties::{Properties, Value};
use enumset::EnumSetType;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::rc::Rc;

pub struct Object {
    pub guid: u128,
    pub name: Name,
    pub is_item: bool,
    pub location: Option<Location>,
    pub properties: Properties,
    pub object_type: Type,
    pub status_component: Option<usize>,
    pub inventory_component: Option<usize>,
    pub(super) names: Rc<Names>,
}

impl Object {
    pub fn new(
        guid: u128,
        name: Name,
        is_item: bool,
        location: Option<Location>,
        properties: Properties,
        names: Rc<Names>,
    ) -> Self {
        lazy_static! {
            static ref EGG_RE: Regex = Regex::new(r"Egg.*Fertilized").unwrap();
        }
        let class = &names[name.id];
        let object_type = match 1 {
            _ if is_item => {
                if EGG_RE.is_match(&class) {
                    Type::FertilizedEgg
                } else {
                    Type::Item
                }
            }
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

        let status_name = names.get_name_id("MyCharacterStatusComponent").unwrap();
        let inventory_name = names.get_name_id("MyInventoryComponent").unwrap();

        let status_component = match properties.props.get(status_name) {
            Some(index) => match index[0].value {
                Value::Int(ind) => Some(ind as usize),
                _ => None,
            },
            None => None,
        };

        let inventory_component = match properties.props.get(inventory_name) {
            Some(index) => match index[0].value {
                Value::Int(ind) => Some(ind as usize),
                _ => None,
            },
            None => None,
        };

        Object {
            guid,
            name,
            is_item,
            location,
            properties,
            object_type,
            names,
            status_component,
            inventory_component,
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
