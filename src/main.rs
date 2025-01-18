use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

mod add;
mod init;
mod list;
mod music;
mod remove;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a song (or all the songs in a directory) to the store.
    Add {
        path: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },
    /// Initialize a new store.
    Init {
        /// Force initialization even if the store already exists
        /// (this will delete the existing files in the directory)
        #[arg(short, long)]
        force: bool,
    },
    /// Remove a song (songs) from the store by its hash id.
    Remove(Remove),
    /// List all the songs in the store.
    List,
}

#[derive(Args)]
struct Remove {
    /// The hash id of the song (songs) to remove
    #[arg(required= true, num_args = 1..)]
    id: Vec<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { force }) => init::init(force),
        Some(Commands::Add { path, recursive }) => add::add(path, recursive),
        Some(Commands::Remove(args)) => remove::remove(&args.id),
        Some(Commands::List) => list::list(),
        None => println!("No command provided."),
    }
}
