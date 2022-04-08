use crate::utils;
use serde::Deserialize;
use std::io::prelude::*;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Deserialize)]
struct ZettelNote {
    title: String,
    tags: String,
    markdown: String,
}

pub fn build() {
    utils::create_directory(utils::SITE_DIRECTORY);
    utils::create_directory(utils::ZETTEL_DIRECTORY);

    let html_string = fs::read_to_string("zettel.html").expect("Error while reading [zettel.html]");

    let notes = fs::read_dir("zettelkasten")
        .expect("directory [zettelkasten] is not found")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Error collecting notes");

    let mut index_markdown = String::new();

    utils::copy_file("zettelstyle.css", "site/zettelkasten/zettelstyle.css");

    for note in notes {
        let path = note;

        if let Some(_) = path.extension() {
            let contents =
                fs::read_to_string(path.clone()).expect("Something went wrong reading the file");

            let zettel_note: ZettelNote = toml::from_str(contents.as_str())
                .expect(format!("error while parsing: {:?}", path).as_str());

            let mut html_output = html_string.clone();
            html_output = html_output.replace("$title", zettel_note.title.as_str());
            html_output = html_output.replace("$tags", zettel_note.tags.as_str());
            html_output = html_output.replace(
                "$content",
                utils::to_html(zettel_note.markdown.as_str()).as_str(),
            );

            let file_name = path.file_stem().unwrap();
            let mut out_path = PathBuf::from(utils::ZETTEL_DIRECTORY);

            out_path.push(file_name);
            out_path.set_extension("html");

            index_markdown.push('[');
            index_markdown.push_str(zettel_note.title.as_str());
            index_markdown.push(']');
            index_markdown.push('(');
            index_markdown.push_str(file_name.to_str().unwrap());
            index_markdown.push_str(".html)  \r");

            let mut file = fs::File::create(out_path).expect("Error creating file");
            file.write_all(&html_output.into_bytes()[..])
                .expect("Error writing to file");
        }
    }

    let mut index_html = html_string.clone();
    index_html = index_html.replace("$title", "zettelkasten");
    index_html = index_html.replace("$tags", "");
    index_html = index_html.replace("$content", utils::to_html(index_markdown.as_str()).as_str());

    let mut index_file = fs::File::create("site/zettelkasten/index.html").unwrap();
    index_file
        .write_all(&index_html.into_bytes()[..])
        .expect("error writing index file");
}
