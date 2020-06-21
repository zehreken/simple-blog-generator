use std::fs;
use std::path::Path;

pub fn create_directory(path: &str) {
    if Path::new(path).exists() {
        println!("Directory [{}] already exists", path);
    } else {
        let r = fs::create_dir(path);
        match r {
            Ok(_) => println!("Successfully created directory [{}]", path),
            Err(error) => panic!("Couldn't create directory [{}], {}", path, error),
        }
    }
}
