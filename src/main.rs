#![allow(unused)]

use std::{path::PathBuf, io::{self, Read, Write}};
use std::fs::File;
use clap::{Parser, Subcommand};

mod mech;

use mech::Mech;


//* Remove the following line when compiling for production
#[cfg(not(debug_assertions))]
const DEFAULT_DATA_FILE_PATH: &str = "data.json";
#[cfg(debug_assertions)]
const DEFAULT_DATA_FILE_PATH: &str = "test/data.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "DATA FILE")]
    file: Option<PathBuf>,

    #[arg(short, long, help = "Lists all mechs without launching main application")]
    list: bool,
}


fn main() {
    let mut args = Args::parse();
    if args.file.is_none() {
        args.file = Some(PathBuf::from(DEFAULT_DATA_FILE_PATH));
    }

    let mut data_file = match File::open(args.file.as_ref().unwrap()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening data file: {}", e);
            return;
        }
    };

    if args.list {
        let mechs = match get_mechs(&mut data_file) {
            Ok(mechs) => mechs,
            Err(e) => {
                eprintln!("Error reading data file: {}", e);
                return;
            }
        };

        for mech in mechs {
            println!("[{}] {}\nWeight: {} tons\nSpeed: {} km/h\n", mech.model(), mech.name(), mech.weight(), mech.speed());
        }

        return;
    }
}


fn get_mechs(data_file: &mut File) -> io::Result<Vec<Mech>> {
    let mut data = String::new();
    data_file.read_to_string(&mut data)?;

    Ok(serde_json::from_str(&data)?)
}
