use std::env;
use clap::{Parser, Subcommand};
// mod task;

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,

}

#[derive(Parser, Debug)]
#[clap(name = "Rusty Task Manager", version = "0.1.0", author = "moxak")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[clap()]
        task: String,
    },
    Done {
        #[clap()]
        id: u32,
    },
    Delete {
        #[clap()]
        id: u32,
    },
    List,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
