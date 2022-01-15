use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use serde::Deserialize;

use crate::utils;

#[derive(Debug, Deserialize)]
pub struct Post {
    pub layout: String,
    pub title: String,
    pub created: String,
    pub updated: String,
    pub markdown: String,
}

impl Post {
    pub fn new(content: &str) -> Self {
        let post: Post =
            toml::from_str(content).expect(format!("Error deserializing content").as_str());
        post
    }
}

pub fn build() {
    utils::create_directory(utils::SITE_DIRECTORY);

    let head_string = fs::read_to_string("head.html");
    let head_string = match head_string {
        Ok(file) => file,
        Err(error) => panic!("Error reading [head.html]: {}", error),
    };

    let mut entries = fs::read_dir("posts")
        .expect("Directory [posts] is not found")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Error collecting entries");

    // Sort by file name
    entries.sort();
    entries.reverse();

    utils::copy_file("style.css", "site/style.css");

    let mut index_markdown = String::new();

    // Add pages
    index_markdown.push_str("[$* About](about.html)<br>");
    index_markdown.push_str("[$* Projects](projects.html)<br><br>");

    for entry in entries {
        let path = entry;
        // This filters .DS_Store
        if let Some(_) = path.extension() {
            let contents = fs::read_to_string(&path)
                .expect(format!("Error reading file [{:?}]", path).as_str());

            let post: Post = Post::new(contents.as_str());

            let mut html_output = head_string.clone();
            html_output = html_output.replace("$title", post.title.as_str());
            html_output = html_output.replace(
                "$updated",
                format!("created on {}, edited on {}", post.created, post.updated).as_str(),
            );
            html_output =
                html_output.replace("$content", utils::to_html(post.markdown.as_str()).as_str());

            let file_name = path.file_stem().unwrap();
            let mut out_path = PathBuf::from(utils::SITE_DIRECTORY);

            if post.layout == "post" {
                index_markdown.push('[');
                index_markdown.push_str("$*");
                index_markdown.push(' ');
                index_markdown.push_str(post.title.as_str());
                index_markdown.push(']');
                index_markdown.push('(');
                index_markdown.push_str(file_name.to_str().unwrap());
                index_markdown.push_str(".html");
                index_markdown.push_str(")<br>");
            }

            out_path.push(file_name);
            out_path.set_extension("html");

            println!("{:?}", &out_path);
            let mut file = fs::File::create(out_path).expect("Error creating file");
            file.write_all(&html_output.into_bytes()[..])
                .expect("Error writing to file");
        }
    }

    let mut index_html = head_string.clone();
    index_html = index_html.replace("$title", "");
    index_html = index_html.replace("$updated", "");
    index_html = index_html.replace("$content", utils::to_html(index_markdown.as_str()).as_str());
    index_html = index_html.replace("$*", "<span> * </span>"); // 'Thin space' character is used before and after asterisk
    let mut index_file = fs::File::create("site/index.html").unwrap();
    index_file.write_all(&index_html.into_bytes()).unwrap()
}
