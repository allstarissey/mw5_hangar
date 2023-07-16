#![allow(unused)]

use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod mech;


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

    match args.command {
        Some(c) => match c {
            Commands::List => {},
            Commands::Add => {},
            Commands::Remove => {},
        },
        None => {},
    }
}
