use arksavefile::{ArkSave, Object};
use serde_json;
use std::env;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = ArkSave::open(&args[1])?;
        let black_pearl = *file.get_name_id("PrimalItemResource_BlackPearl_C").unwrap();
        let all_objects = file.read_objects()?;
        let objects: Vec<&Object> = all_objects
            .iter()
            .filter(|o| o.name.id == black_pearl)
            .collect::<Vec<&Object>>();
        serde_json::to_writer(stdout(), &objects)?;
    } else {
        println!("Usage: {} <savefile>", args[0]);
    }
    Ok(())
}
