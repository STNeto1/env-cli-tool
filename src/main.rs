use clap::Parser;
use cli::{Action, Args};

mod cli;

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Init => println!("init"),
        Action::Add(props) => println!("add -> k:{} v:{}", props.key, props.value),
        Action::List => println!("list"),
        Action::Remove(props) => println!("remove -> k:{}", props.key),
    };
}
