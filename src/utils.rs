use std::fs;
use std::path::Path;

pub fn create_dir(path: &str) {
    let dir = Path::new(path);
    if !dir.exists() {
        println!("Creating {}...", path);
        fs::create_dir(dir).unwrap();
        println!("{} created.", path);
    } else {
        println!("{} already exists. Skipping...", path);
    }
}
