use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod add;
mod init;
mod music;
mod remove;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Push a song (or all the songs in a directory) to the store.
    Push {
        path: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },
    /// Initialize a new store
    Init {
        /// Force initialization even if the store already exists
        /// (this will delete the existing files in the directory)
        #[arg(short, long)]
        force: bool,
    },
    Remove {
        id: String,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Subcommands::Init { force }) => init::init(force),
        Some(Subcommands::Push { path, recursive }) => add::add(path, recursive),
        Some(Subcommands::Remove { id }) => println!("Work in progress..."),
        None => println!("No command provided."),
    }
}
