use rusqlite::Connection;
use std::fs;

pub fn remove(ids: &Vec<String>) {
    for id in ids {
        remove_song(id);
    }
}

fn remove_song(id: &str) {
    fs::remove_file(format!("store/{}", id)).unwrap();
    println!("Song file with id {} removed.", id);
    let conn = Connection::open(".sailer/sailer.db").unwrap();
    let mut stmt = conn.prepare("DELETE FROM songs WHERE id = ?").unwrap();
    stmt.execute([id]).unwrap();
    println!("Song data with id {} removed.", id);
}
