use arksavefile::ArkSave;
use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = ArkSave::open(&args[1])?;
        file.read_all()?;
    } else {
        println!("Usage: {} <savefile>", args[0]);
    }
    Ok(())
}
