use arksavefile::{ArkSave, Type};
use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = ArkSave::open(&args[1])?;
        let names = file.read_names()?;
        let objects = file.read_objects(&names)?;
        for o in objects {
            if o.object_type == Type::FertilizedEgg {
                println!("---");
                println!("{} {:?}", o.name.id, o.location);
                for p in o.properties.props.values() {
                    if p.len() == 1 {
                        println!("{} {:?}", &names[p[0].name.id], p[0].value);
                    } else {
                        print!("{}", &names[p[0].name.id]);
                        let mut i = 0;
                        for v in p {
                            while i < v.ind {
                                print!(" 0");
                                i = i + 1;
                            }
                            print!(" {}", v.value);
                            i = i + 1;
                        }
                        println!();
                    }
                }
            }
        }
    } else {
        println!("Usage: {} <savefile>", args[0]);
    }
    Ok(())
}
