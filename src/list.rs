use rusqlite::Connection;

use crate::music;

pub fn list() {
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
    for song in songs {
        let (id, title, artist, album) = song.unwrap();
        let music = music::Music::new(&id, Some(&title), Some(&artist), Some(&album));
        println!("{}", music.to_string());
    }
}
