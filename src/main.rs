use std::fs::{self, File};

use anyhow::{bail, Result};
use clap::Parser;
use cli::{Action, Args};

mod cli;

const PATH: &str = "config.toml";

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

    let mut _output = File::create(PATH)?;
    return Ok(());
}
