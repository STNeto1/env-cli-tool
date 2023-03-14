use anyhow::{bail, Result};
use clap::Parser;
use cli::{Action, Args};
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
        Action::Add(props) => println!("add -> k:{} v:{}", props.key, props.value),
        Action::List => println!("list"),
        Action::Remove(props) => println!("remove -> k:{}", props.key),
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
