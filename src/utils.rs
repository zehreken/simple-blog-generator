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

pub fn copy_file(from: &str, to: &str) {
    if Path::new(from).exists() {
        let r = fs::copy(Path::new(from), Path::new(to));
        match r {
            Ok(_) => println!("Successfuly copied [{}] to [{}]", from, to),
            Err(error) => println!("Error copying file: {}", error),
        }
    } else {
        println!("File [{}] does not exist", from);
    }
}

pub fn copy_directory(from: &str, to: &str) {
    if Path::new(from).exists() {
        // copy directory
    } else {
        println!("Directory [{}] does not exist", from);
    }
}
