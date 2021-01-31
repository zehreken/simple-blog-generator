use crate::utils;
use serde::Deserialize;
use std::io::prelude::*;
use std::{fs, io, path::PathBuf};

const ZETTEL_DIRECTORY: &str = "site/zettelkasten";

pub fn run() {
    utils::create_directory(ZETTEL_DIRECTORY);

    let html_string = fs::read_to_string("zettel.html").expect("Error while reading [zettel.html]");

    let notes = fs::read_dir("zettelkasten")
        .expect("directory [zettelkasten] is not found")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Error collecting notes");

    let mut index_markdown = String::new();

    for note in notes {
        let path = note;

        if let Some(_) = path.extension() {
            let contents =
                fs::read_to_string(path.clone()).expect("Something went wrong reading the file");

            let zettel_note: ZettelNote = toml::from_str(contents.as_str())
                .expect(format!("error while parsing: {:?}", path).as_str());

            let mut html_output = html_string.clone();
            html_output = html_output.replace("$title", zettel_note.title.as_str());
            html_output = html_output.replace(
                "$content",
                utils::to_html(zettel_note.markdown.as_str()).as_str(),
            );

            let file_name = path.file_stem().unwrap();
            let mut out_path = PathBuf::from(ZETTEL_DIRECTORY);

            out_path.push(file_name);
            out_path.set_extension("html");

            let mut file = fs::File::create(out_path).expect("Error creating file");
            file.write_all(&html_output.into_bytes()[..])
                .expect("Error writing to file");
        }
    }
}

#[derive(Debug, Deserialize)]
struct ZettelNote {
    title: String,
    markdown: String,
}
