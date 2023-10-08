
use std::fs;

fn main() {
    

    let paths = fs::read_dir("/bin/").unwrap();

    for dirEntry in paths {
        let path = dirEntry.unwrap().path();
        
        let bytes = std::fs::read(path.clone()).unwrap();
        let hash = sha256::digest_bytes(&bytes);

        println!("{},{}",path.display(),hash);
    }
}
