use rusqlite::Connection;
use std::fs;
use std::path::Path;

use crate::utils::create_dir;

pub fn init(force: &bool) {
    if *force {
        println!(
            "Force initialization.
Existing .sailer and imported music files will be deleted.
Continue? (y/n)"
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            println!("Force initialization canceled.");
            return;
        } else {
            println!("Deleting existing .sailer directory...");
            fs::remove_dir_all(Path::new(".sailer")).unwrap();
        }
    }

    create_dir(".sailer");
    create_dir(".sailer/store");

    let db_file = Path::new(".sailer/sailer.db");
    if !db_file.exists() {
        println!("Creating .sailer/sailer.db file...");
        let conn = Connection::open(db_file).unwrap();
        match conn.execute(
            "CREATE TABLE songs (
                    id TEXT PRIMARY KEY,
                    title TEXT,
                    artist TEXT,
                    album TEXT
                )",
            (),
        ) {
            Ok(_) => println!("Songs database created."),
            Err(e) => println!("Error creating songs database: {}", e),
        };
        println!(".sailer/sailer.db file created.");
    } else {
        println!(".sailer/sailer.db file already exists. Skipping...");
    }
}
