mod config;
mod record;

use config::TomlConfig;
use std::collections::HashMap;
use std::fs;

fn main() {
    let config: TomlConfig = TomlConfig::load().expect("Failed to load config");
    let mut hashes = HashMap::new();

    let paths = fs::read_dir(config.get_dir_baseline()).unwrap();

    for dir in paths {
        let path = dir.unwrap().path();

        let filename: String = path.file_name().unwrap().to_str().unwrap().to_string();

        match std::fs::read(path.clone()) {
            Ok(bytes) => {
                let hash = sha256::digest(&bytes);
                hashes.insert(filename, hash.clone());
            }
            Err(_) => (),
        }
    }

    let paths = fs::read_dir(config.get_dir_target()).unwrap();

    for dir in paths {
        let path = dir.unwrap().path();

        let filename: String = path.file_name().unwrap().to_str().unwrap().to_string();

        match std::fs::read(path.clone()) {
            Ok(bytes) => {
                let hash = sha256::digest(&bytes);
                match hashes.get(&filename) {
                    Some(good_hash) => {
                        if good_hash != &hash {
                            println!("{} missed match hash!", filename);
                        }
                    }
                    None => println!("{} not found in known good!", filename),
                }
            }
            Err(_) => (),
        }
    }
}
