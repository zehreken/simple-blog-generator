use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};

fn main() {
    let head_string = fs::read_to_string("head.txt");
    let head_string = match head_string {
        Ok(file) => file,
        Err(error) => panic!("Error while readin head.txt: {:?}", error),
    };

    let mut entries = fs::read_dir("posts")
        .expect("'posts' directory is not found")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Error collecting entriescar");

    entries.sort();

    let r = fs::copy(Path::new("style.css"), Path::new("site/style.css"));
    let _ = match r {
        Ok(_) => (),
        Err(error) => panic!("Error copying file: {:?}", error),
    };

    let r = fs::create_dir("site");
    let r = match r {
        Ok(_) => (),
        Err(error) => println!("{:?}", error),
    };

    for entry in entries {
        let path = entry;
        // This filters .DS_Store
        if let Some(_) = path.extension() {
            let contents =
                fs::read_to_string(path.clone()).expect("Something went wrong reading the file");

            let mut html_output = head_string.clone();
            html_output = html_output.replace("$content", to_markdown(contents.as_str()).as_str());

            let file_name = path.file_stem().unwrap();
            let mut out_path = PathBuf::from("site");
            out_path.push(file_name);
            out_path.set_extension("html");

            println!("{:?}", out_path.clone());
            let mut file = fs::File::create(out_path).expect("Error creating file");
            file.write_all(&html_output.into_bytes()[..])
                .expect("Error writing to file");
        }
    }
}

fn to_markdown(markdown: &str) -> String {
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
