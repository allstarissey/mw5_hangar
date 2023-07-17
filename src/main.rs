#![allow(unused)]

use std::{path::PathBuf, io::{self, Read, Write}};
use std::fs::File;
use clap::{Parser, Subcommand};

mod mech;

use mech::Mech;


#[cfg(not(debug_assertions))]
const DEFAULT_DATA_FILE_PATH: &str = "data.json";
#[cfg(debug_assertions)]
const DEFAULT_DATA_FILE_PATH: &str = "test/data.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, value_name = "DATA FILE")]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Lists all mechs defined in the data file")]
    List,

    #[command(about = "Adds a new mech to the data file")]
    Add,

    #[command(about = "Removes a mech from the data file")]
    Remove,
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

    match args.command {
        Some(c) => match c {
            Commands::List => {
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
            },
            Commands::Add => {},
            Commands::Remove => {},
        },
        None => {},
    }
}


fn get_mechs(data_file: &mut File) -> io::Result<Vec<Mech>> {
    let mut data = String::new();
    data_file.read_to_string(&mut data)?;

    Ok(serde_json::from_str(&data)?)
}
