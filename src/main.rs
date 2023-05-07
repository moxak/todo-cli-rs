use std::env;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILE_NAME: &str = "tasks.json";

fn save_task(task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = match read_tasks() {
        Ok(tasks) => tasks,
        Err(_) => Vec::new(),
    };

    tasks.push(task.clone());

    let json = serde_json::to_string(&tasks)?;

    let mut file = File::create(FILE_NAME)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if Path::new(FILE_NAME).exists() {
        let mut file = File::open(FILE_NAME)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    } else {
        Err("File not found".into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

fn add_task(task: String) -> Task {
    let id = 1; // TODO: implement ID generation logic
    Task { id, title: task}
}

fn main() {
    let args = Args::parse();
    
    if cfg!(debug_assertions) {
        println!("{:?}", args);
    }

    match args.command {
        Commands::Add { task } => {
            let new_task = add_task(task);
            println!("Added new task: {:?}", new_task);

            match save_task(&new_task) {
                Ok(()) => println!("Task added: {:?}", new_task),
                Err(e) => println!("Failed to save task: {}", e),
            }
        },
        Commands::Done { id } => {},
        Commands::Delete { id } => {},
        Commands::List => {},
    }
}
