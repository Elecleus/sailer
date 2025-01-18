use audiotags::Tag;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::music;

pub fn add(path: &PathBuf, recursive: &bool) {
    // 判断给定路径是否为文件夹
    if path.is_dir() {
        add_all_songs(path, recursive).unwrap();
    } else {
        add_one_song(path);
    }
}

fn add_one_song<T: AsRef<Path>>(path: T) {
    let tag = Tag::new().read_from_path(&path).unwrap();
    let music = music::Music {
        id: &get_hash(&path),
        title: tag.title(),
        artist: tag.artist(),
        album: match tag.album() {
            Some(album) => Some(album.title),
            None => None,
        },
    };
    let conn = Connection::open(".sailer/sailer.db").unwrap();
    match conn.execute(
        "INSERT INTO songs (id, title, artist, album) VALUES (?1, ?2, ?3, ?4)",
        params![music.id, music.title, music.artist, music.album],
    ) {
        Ok(_) => println!("Song added to database."),
        Err(e) => println!("Error adding song to database: {}", e),
    };
    let store_path = Path::new(".sailer/store").join(music.id);
    std::fs::copy(&path, store_path).unwrap();
    println!(
        "{} copied to store.",
        path.as_ref().file_name().unwrap().to_str().unwrap()
    );
}

fn add_all_songs<T: AsRef<Path>>(path: T, recursive: &bool) -> std::io::Result<()> {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();

        if path.is_dir() {
            if *recursive {
                add_all_songs(&path, recursive)?;
            }
        } else {
            add_one_song(&path);
        }
    }

    Ok(())
}

fn get_hash<T: AsRef<Path>>(path: T) -> String {
    let path = path.as_ref();
    let mut file = std::fs::File::open(path).unwrap();
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    format!("{:x}", hasher.finalize())[..8].to_string()
}
