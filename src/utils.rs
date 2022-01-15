use std::fs;
use std::path::Path;

pub const SITE_DIRECTORY: &str = "site";
pub const ZETTEL_DIRECTORY: &str = "site/zettelkasten";

pub fn to_html(markdown: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

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

pub fn _copy_directory(from: &str, _to: &str) {
    if Path::new(from).exists() {
        // copy directory
    } else {
        println!("Directory [{}] does not exist", from);
    }
}
