use clap::{Parser, Subcommand};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Target store to operate on
    #[arg(short, long, value_name = "STORE")]
    target: Option<String>,

    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Push a song (or all the songs in a directory) to the store.
    Push {
        #[arg(short, long)]
        list: bool,
    },
    /// Check the status of a store
    Check {
        #[arg(short, long)]
        list: bool,
    },
    /// Initialize a new store
    Init {
        /// Force initialization even if the store already exists
        /// (this will delete the existing files in the directory)
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Subcommands::Init { force }) => init(force),
        Some(_) => {
            println!("Not printing testing lists...");
        }
        None => {}
    }
}

fn init(force: &bool) {
    let sailer_dir = Path::new(".sailer");
    println!("{}", sailer_dir.display());
    let existing = sailer_dir.exists();
    if existing {
        if *force {
            println!(
                "Force initialization.
Existing .sailer will be deleted.
Continue? (y/n)"
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" {
                println!("Aborted.");
                return;
            } else {
                println!("Deleting existing .sailer directory...");
                fs::remove_dir_all(sailer_dir).unwrap();
            }
        } else {
            println!(".sailer directory exists.\nSkip initialization.");
            return;
        }
    } else {
        println!(".sailer directory does not exist.");
    }
    println!("Creating .sailer directory...");
    fs::create_dir(sailer_dir).unwrap();
    println!("Initialization complete.");
    return;
}
