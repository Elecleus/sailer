use rusqlite::Connection;
use std::os::unix::fs::symlink;

use crate::music;
use crate::utils::create_dir;

const CLASSIFY_BY: [&str; 3] = ["all", "artist", "album"];

const ARTIST_DIVIDE_SYMBOLS: [&str; 2] = ["/", "、"];

// Possible path: ./all/<name>
// Possible path: ./artist/<artist>/<name>

pub fn classify(by: &str, list: &bool) {
    if *list {
        println!("Available classification methods:");
        for item in CLASSIFY_BY.iter() {
            println!("{}", item);
        }
        return;
    }
    if !CLASSIFY_BY.contains(&by) {
        eprintln!("Invalid classification method.");
        return;
    }
    let conn = Connection::open(".sailer/sailer.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title, artist, album FROM songs")
        .unwrap();
    let songs = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .unwrap();
    create_dir(&format!("./{}", by));
    for song in songs {
        let (id, title, artist, album) = song.unwrap();
        let music = music::Music::new(&id, Some(&title), Some(&artist), Some(&album));

        // former code copied from src/list.rs, next code are new
        match by {
            "all" => {
                symlink_to_dir(
                    "../.sailer/store",
                    "./all",
                    &format!("{}.mp3", music.to_string_no_id()),
                    music.get_id(),
                );
            }
            "artist" => {
                let artists = divide_by_symbols(music.get_artist());
                for artist in artists {
                    symlink_to_dir(
                        "../../.sailer/store",
                        &format!("./artist/{}", artist),
                        &format!("{}.mp3", music.to_string_no_id()),
                        music.get_id(),
                    );
                }
            }
            "album" => {
                symlink_to_dir(
                    "../../.sailer/store",
                    &format!("./album/{}", music.get_album()),
                    &format!("{}.mp3", music.to_string_no_id()),
                    music.get_id(),
                );
            }
            _ => {
                eprintln!("Invalid classification method.");
                return;
            }
        }
    }
}

fn divide_by_symbols(s: &str) -> Vec<String> {
    let mut result = vec![s.to_string()];
    for symbol in ARTIST_DIVIDE_SYMBOLS.iter() {
        let mut temp = Vec::new();
        for item in result.iter() {
            temp.extend(item.split(symbol).map(|x| x.to_string()));
        }
        result = temp;
    }
    result
}

fn symlink_to_dir(from_dir: &str, to_dir: &str, sym_link_name: &str, id: &str) {
    let from_path = format!("{}/{}", from_dir, id);

    create_dir(&to_dir);

    let link_path = format!("{}/{}", to_dir, sym_link_name);
    symlink(&from_path, &link_path).unwrap();
}
