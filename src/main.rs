use arksavefile::{ArkParser, Entry, MMappedReader, Type};
use serde::Serialize;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Wild<'a> {
    tameable: bool,
    is_female: bool,
    class_name: &'a str,
    x: f32,
    y: f32,
    z: f32,
    base_stats: Vec<i32>,
    base_level: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Tamed<'a> {
    is_female: bool,
    name: &'a str,
    class_name: &'a str,
    x: f32,
    y: f32,
    z: f32,
    base_stats: Vec<i32>,
    tamed_stats: Vec<i32>,
    base_level: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Baby<'a> {
    parent: &'a str,
    class_name: &'a str,
    base_stats: Vec<i32>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = MMappedReader::open(&args[1])?;
        let file = ArkParser::read(&mut file)?;
        fs::create_dir_all(&file.map)?;
        write_wild(&file)?;
        write_tamed(&file)?;
        write_nursery(&file)?;
        write_cryopods(&file)?;
    } else {
        println!("Usage: {} <savefile>", args[0]);
    }
    Ok(())
}

fn stats(entry: &Entry, name: &usize) -> Vec<i32> {
    let mut stats = entry
        .status_component()
        .unwrap()
        .properties
        .get_vec_int(name);
    stats.resize_with(12, || 0);
    stats
}

fn write_wild(file: &ArkParser) -> Result<()> {
    let is_female = file.get_name_id("bIsFemale").unwrap();
    let taming_disabled = file.get_name_id("bForceDisablingTaming").unwrap();
    let base_levels = file.get_name_id("NumberOfLevelUpPointsApplied").unwrap();
    let base_level = file.get_name_id("BaseCharacterLevel").unwrap();
    let entries: Vec<Wild> = file
        .entries()
        .iter()
        .filter(|o| o.object_type == Type::WildCreature)
        .filter(|o| o.location().is_some())
        .map(|o| {
            let loc = o.location().unwrap();
            Wild {
                is_female: o.properties().get_bool(is_female).unwrap_or(false),
                tameable: !o.properties().get_bool(taming_disabled).unwrap_or(false),
                class_name: file.get_name(o.name().id),
                x: loc.x,
                y: loc.y,
                z: loc.z,
                base_level: o
                    .status_component()
                    .unwrap()
                    .properties
                    .get_i32(base_level)
                    .or(Some(1))
                    .unwrap(),
                base_stats: stats(o, base_levels),
            }
        })
        .collect();
    let file = BufWriter::new(File::create(format!("{}/wild.json", file.map))?);
    serde_json::to_writer(file, &entries)?;
    Ok(())
}

fn write_tamed(file: &ArkParser) -> Result<()> {
    let is_female = file.get_name_id("bIsFemale").unwrap();
    let base_levels = file.get_name_id("NumberOfLevelUpPointsApplied").unwrap();
    let base_level = file.get_name_id("BaseCharacterLevel").unwrap();
    let tamed_levels = file
        .get_name_id("NumberOfLevelUpPointsAppliedTamed")
        .unwrap();
    let tamed_name = file.get_name_id("TamedName").unwrap();

    let entries: Vec<Tamed> = file
        .entries()
        .iter()
        .filter(|o| o.object_type == Type::TamedCreature)
        .filter(|o| o.location().is_some())
        .map(|o| {
            let loc = o.location().unwrap();
            Tamed {
                is_female: o.properties().get_bool(is_female).unwrap_or(false),
                name: o.properties().get_str(tamed_name).unwrap_or(""),
                class_name: file.get_name(o.name().id),
                x: loc.x,
                y: loc.y,
                z: loc.z,
                base_level: o
                    .status_component()
                    .unwrap()
                    .properties
                    .get_i32(base_level)
                    .unwrap_or(1),
                base_stats: stats(o, base_levels),
                tamed_stats: stats(o, tamed_levels),
            }
        })
        .collect();
    let file = BufWriter::new(File::create(format!("{}/tames.json", file.map))?);
    serde_json::to_writer(file, &entries)?;
    Ok(())
}

fn write_nursery(file: &ArkParser) -> Result<()> {
    let base_levels_opt = file.get_name_id("GestationEggNumberOfLevelUpPointsApplied");
    if base_levels_opt.is_none() {
        fs::write("nursery.json", "[]")?;
        return Ok(());
    }

    let base_levels = base_levels_opt.unwrap();
    let mother = file.get_name_id("TamedName").unwrap();
    let parents = file.get_name_id("CustomItemDescription").unwrap();
    let egg_levels = file.get_name_id("EggNumberOfLevelUpPointsApplied").unwrap();

    let entries: Vec<Baby> = file
        .entries()
        .iter()
        .filter(|o| {
            (o.object_type == Type::TamedCreature && o.properties().contains_id(base_levels))
                || o.object_type == Type::FertilizedEgg
        })
        .map(|o| {
            if o.object_type == Type::TamedCreature {
                let mut base_stats = o.properties().get_vec_int(base_levels);
                base_stats.resize_with(12, || 0);
                Baby {
                    parent: o.properties().get_str(mother).unwrap_or_default(),
                    class_name: file.get_name(o.name().id),
                    base_stats,
                }
            } else {
                let mut base_stats = o.properties().get_vec_int(egg_levels);
                base_stats.resize_with(12, || 0);
                Baby {
                    parent: o.properties().get_str(parents).unwrap(),
                    class_name: file.get_name(o.name().id),
                    base_stats,
                }
            }
        })
        .collect();
    let file = BufWriter::new(File::create(format!("{}/nursery.json", file.map))?);
    serde_json::to_writer(file, &entries)?;
    Ok(())
}

fn write_cryopods(file: &ArkParser) -> Result<()> {
    let cryopod = *file.get_name_id("PrimalItem_WeaponEmptyCryopod_C").unwrap();

    let entries: Vec<&Entry> = file
        .entries()
        .iter()
        .filter(|o| o.name().id == cryopod)
        .collect();
    let file = BufWriter::new(File::create(format!("{}/cryopods.json", file.map))?);
    serde_json::to_writer_pretty(file, &entries)?;
    Ok(())
}
