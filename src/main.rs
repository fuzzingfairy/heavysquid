
use std::fs;
use std::collections::HashMap;

fn main() {

    let mut hashes = HashMap::new();

    let paths = fs::read_dir("/bin/").unwrap();

    for dirEntry in paths {
        let path = dirEntry.unwrap().path();
        
        let fileStr :String= path.file_name().unwrap().to_str().unwrap().to_string();

        match std::fs::read(path.clone()) {
            Ok(bytes) => {
                let hash = sha256::digest(&bytes);
                hashes.insert(fileStr,hash.clone());
            },
            Err(error) => ()
        }
    }



    let paths = fs::read_dir("testbin/").unwrap();

    for dirEntry in paths {
        let path = dirEntry.unwrap().path();
        
        let fileStr :String= path.file_name().unwrap().to_str().unwrap().to_string();

        match std::fs::read(path.clone()) {
            Ok(bytes) => {
                let hash = sha256::digest(&bytes);
                match hashes.get(&fileStr) {
                    Some(goodHash) => {
                        if goodHash != &hash {
                            println!("{} missed match hash!",fileStr);
                        }
                    },
                    None => println!("{} not found in known good!",fileStr)
                }
            },
            Err(error) => ()
        }
    }


}
