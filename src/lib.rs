use crate::EntryType::*;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "PATH", help = "Search paths", default_value = ".")]
    paths: Vec<String>,

    #[arg(
        value_name = "NAME",
        short,
        long = "name",
        help = "Name",
        value_delimiter = ',',
        num_args = 1..
    )]
    names: Vec<Regex>,

    #[arg(
        value_name = "TYPE",
        short = 't',
        long = "type",
        help = "Entry type",
        value_delimiter = ',',
        num_args = 1..,
        value_enum
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
enum EntryType {
    #[clap(name = "d")]
    Dir,

    #[clap(name = "f")]
    File,

    #[clap(name = "l")]
    Link,
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                Dir => entry.file_type().is_dir(),
                File => entry.file_type().is_file(),
                Link => entry.file_type().is_symlink(),
            })
    };
    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };
    args.paths
        .iter()
        .flat_map(WalkDir::new)
        .filter_map(|entry| {
            if let Err(ref err) = entry {
                eprintln!("{err}");
            }
            entry.ok()
        })
        .filter(type_filter)
        .filter(name_filter)
        .for_each(|entry| println!("{}", entry.path().display()));
    Ok(())
}
