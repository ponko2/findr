use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;

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
    dbg!(args);
    Ok(())
}
