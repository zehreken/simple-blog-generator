use crate::{context::BuildContext, utils};
use serde::Deserialize;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Debug, Deserialize)]
struct ZettelNote {
    title: String,
    tags: String,
    markdown: String,
}

pub fn build(ctx: &BuildContext) {
    let zettel_out = ctx.output_dir.join("zettelkasten");

    if !ctx.output_dir.exists() {
        fs::create_dir_all(&ctx.output_dir).expect("Failed to create output directory");
    }
    if !zettel_out.exists() {
        fs::create_dir_all(&zettel_out).expect("Failed to create zettelkasten output directory");
    }

    if !ctx.zettelkasten_dir.exists() {
        println!(
            "Directory {:?} not found, skipping zettelkasten build",
            ctx.zettelkasten_dir
        );
        return;
    }

    let html_string = &ctx.templates.zettel;

    let mut notes = fs::read_dir(&ctx.zettelkasten_dir)
        .unwrap_or_else(|_| panic!("Directory {:?} not found", ctx.zettelkasten_dir))
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Error collecting notes");

    notes.sort();

    fs::write(
        zettel_out.join("zettelstyle.css"),
        &ctx.templates.zettel_css,
    )
    .expect("Failed to write zettelstyle.css");

    let mut index_markdown = String::new();

    for note in notes {
        if note.extension().is_none() {
            continue;
        }

        let contents =
            fs::read_to_string(&note).unwrap_or_else(|_| panic!("Error reading file {:?}", note));

        let zettel_note: ZettelNote =
            toml::from_str(&contents).unwrap_or_else(|_| panic!("Error parsing {:?}", note));

        let mut html_output = html_string.clone();
        html_output = html_output.replace("$title", &zettel_note.title);
        html_output = html_output.replace("$tags", &zettel_note.tags);
        html_output = html_output.replace("$content", &utils::to_html(&zettel_note.markdown));

        let file_name = note.file_stem().unwrap();
        let mut out_path = zettel_out.clone();
        out_path.push(file_name);
        out_path.set_extension("html");

        index_markdown.push_str(&format!(
            "[{}]({}.html)  \r",
            zettel_note.title,
            file_name.to_str().unwrap()
        ));

        let mut file = fs::File::create(out_path).expect("Error creating file");
        file.write_all(html_output.as_bytes())
            .expect("Error writing to file");
    }

    let mut index_html = html_string.clone();
    index_html = index_html.replace("$title", "zettelkasten");
    index_html = index_html.replace("$tags", "");
    index_html = index_html.replace("$content", &utils::to_html(&index_markdown));

    let mut index_file =
        fs::File::create(zettel_out.join("index.html")).expect("Failed to create zettel index");
    index_file
        .write_all(index_html.as_bytes())
        .expect("Error writing zettel index");
}
