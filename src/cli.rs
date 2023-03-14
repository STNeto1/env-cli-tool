use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, name = "cli", bin_name= "cli")]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Init,
    Add(AddArgs),
    List,
    Remove(RemoveArgs),
}

#[derive(Debug, clap::Args)]
#[command()]
pub struct AddArgs {
    #[arg(long, short = 'k', help = "Key to be stored")]
    pub key: String,

    #[arg(long, short = 'v', help = "Value to be stored")]
    pub value: String,
}

#[derive(Debug, clap::Args)]
#[command()]
pub struct RemoveArgs {
    #[arg(long, short = 'k')]
    pub key: String,
}
