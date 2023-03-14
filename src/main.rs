use anyhow::{bail, Result};
use clap::Parser;
use cli::{Action, AddArgs, Args, RemoveArgs};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};
use store::DataStore;

mod cli;
mod store;

const PATH: &str = "config.json";

fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Init => init()?,
        Action::Add(props) => add_to_store(&props)?,
        Action::List => list()?,
        Action::Remove(props) => remove_from_store(&props)?,
    };

    return Ok(());
}

fn init() -> Result<()> {
    if fs::read_to_string(PATH).is_ok() {
        bail!("file already exists");
    }

    let data = DataStore {
        keys: HashMap::new(),
    };
    let serialized_data = serde_json::to_string_pretty(&data)?;

    let mut output = File::create(PATH)?;
    write!(output, "{}", serialized_data)?;

    return Ok(());
}

fn read_raw_config_file() -> Result<String> {
    let file = fs::read_to_string(PATH)?;
    return Ok(file);
}

fn list() -> Result<()> {
    let raw = read_raw_config_file()?;

    let data: DataStore = serde_json::from_str(&raw)?;
    for entry in data.keys {
        println!("{}={}", entry.0, entry.1);
    }

    return Ok(());
}

fn add_to_store(data: &AddArgs) -> Result<()> {
    let raw = read_raw_config_file()?;

    let mut data_store: DataStore = serde_json::from_str(&raw)?;
    data_store.add(&data)?;

    let serialized_data = serde_json::to_string_pretty(&data_store)?;
    let mut output = File::create(PATH)?;
    write!(output, "{}", serialized_data)?;

    return Ok(());
}

fn remove_from_store(data: &RemoveArgs) -> Result<()> {
    let raw = read_raw_config_file()?;

    let mut data_store: DataStore = serde_json::from_str(&raw)?;
    data_store.remove(&data)?;

    let serialized_data = serde_json::to_string_pretty(&data_store)?;
    let mut output = File::create(PATH)?;
    write!(output, "{}", serialized_data)?;

    return Ok(());
}
