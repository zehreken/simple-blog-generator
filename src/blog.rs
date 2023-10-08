use crate::utils;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

const STYLE: &str = "style_gray";
const BULLET: &str = "●";

#[derive(Debug, Deserialize)]
pub struct Post {
    pub layout: String,
    pub title: String,
    pub created: String,
    pub updated: String,
    pub tags: String,
    pub markdown: String,
}

impl Post {
    pub fn new(content: &str) -> Self {
        let post: Post =
            toml::from_str(content).expect(format!("Error deserializing content").as_str());
        post
    }
}

fn build_index(tag: &str, posts: &Vec<String>) {
    let head_string = fs::read_to_string("tag_head.html");
    let head_string = match head_string {
        Ok(file) => file,
        Err(error) => panic!("Error reading [tag_head.html]: {}", error),
    };

    let mut index_markdown = String::new();
    for post in posts {
        index_markdown.push('[');
        index_markdown.push_str(format!("${0}", BULLET).as_str());
        index_markdown.push(' ');
        index_markdown.push_str(post);
        index_markdown.push(']');
        index_markdown.push('(');
        index_markdown.push_str("../");
        index_markdown.push_str(post);
        index_markdown.push_str(".html)  \r");
    }

    let mut index_html = head_string.clone();
    index_html = index_html.replace("$title", "");
    index_html = index_html.replace("$date", "");
    index_html = index_html.replace("$tags", "");
    index_html = index_html.replace("$content", utils::to_html(index_markdown.as_str()).as_str());
    index_html = index_html.replace("$●", "<span> ● </span>"); // 'Thin space' character is used before and after asterisk
    let mut index_file =
        fs::File::create(format!("site/tags/{}.html", tag.replace("#", ""))).unwrap();
    index_file.write_all(&index_html.into_bytes()).unwrap()
}

fn get_tags_link(tags: &str) -> String {
    let mut tags_link = String::from("");
    tags_link.push_str("<p>");
    for tag in tags.split(' ') {
        tags_link.push_str(format!("<a href=tags/{}.html>", tag.replace("#", "")).as_str());
        tags_link.push_str(tag);
        tags_link.push_str("</a> ");
    }
    tags_link.push_str("</p>");

    tags_link
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

    let source = format!("{0}.css", STYLE);
    let target = "site/style.css";
    utils::copy_file(source.as_str(), target);

    let mut index_markdown = String::new();

    // Add pages
    index_markdown.push_str("[$● About](about.html)  \r");
    index_markdown.push_str("[$● Projects](projects.html)  \r  \r");
    // End of pages
    let mut prev_year = String::from("");
    let mut posts_by_tag: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries {
        let path = entry;
        // This filters .DS_Store
        if let Some(_) = path.extension() {
            let contents = fs::read_to_string(&path)
                .expect(format!("Error reading file [{:?}]", path).as_str());

            let post: Post = Post::new(contents.as_str());

            let mut html_output = head_string.clone();
            html_output = html_output.replace("$title", post.title.as_str());
            if post.created == post.updated {
                html_output =
                    html_output.replace("$date", format!("created on {}", post.created).as_str());
            } else {
                html_output = html_output.replace(
                    "$date",
                    format!("created on {}, edited on {}", post.created, post.updated).as_str(),
                );
            }

            if post.layout == "post" {
                html_output =
                    html_output.replace("$tags", get_tags_link(post.tags.as_str()).as_str());
            } else if post.layout == "page" {
                html_output = html_output.replace("$tags", "");
            }

            html_output =
                html_output.replace("$content", utils::to_html(post.markdown.as_str()).as_str());

            let file_name = path.file_stem().unwrap();
            let mut out_path = PathBuf::from(utils::SITE_DIRECTORY);

            if post.layout == "post" {
                let year = post.created.split('-').next().unwrap().to_owned();
                let tags = post.tags.split(' ');
                for tag in tags {
                    match posts_by_tag.get_mut(tag) {
                        Some(posts) => posts.push(file_name.to_owned().into_string().unwrap()),
                        None => {
                            posts_by_tag.insert(
                                tag.into(),
                                vec![file_name.to_owned().into_string().unwrap()],
                            );
                        }
                    }
                }
                if prev_year != year {
                    index_markdown.push_str("#### ");
                    index_markdown.push_str(&year);
                    index_markdown.push_str("  \r");
                    prev_year = year.to_owned();
                }
                index_markdown.push('[');
                index_markdown.push_str("$●");
                index_markdown.push(' ');
                index_markdown.push_str(post.title.as_str());
                index_markdown.push(']');
                index_markdown.push('(');
                index_markdown.push_str(file_name.to_str().unwrap());
                index_markdown.push_str(".html)  \r");
            }

            out_path.push(file_name);
            out_path.set_extension("html");

            println!("{:?}", &out_path);
            let mut file = fs::File::create(out_path).expect("Error creating file");
            file.write_all(&html_output.into_bytes()[..])
                .expect("Error writing to file");
        }
    }

    for (tag, posts) in &posts_by_tag {
        println!("[TAG] {}", tag);
        build_index(tag, posts);
        for post in posts {
            println!("{}", post);
            build_index(tag, posts);
        }
    }

    let mut index_html = head_string.clone();
    index_html = index_html.replace("$title", "");
    index_html = index_html.replace("$date", "");
    index_html = index_html.replace("$tags", "");
    index_html = index_html.replace("$content", utils::to_html(index_markdown.as_str()).as_str());
    index_html = index_html.replace("$●", "<span> ● </span>"); // 'Thin space' character is used before and after asterisk
    let mut index_file = fs::File::create("site/index.html").unwrap();
    index_file.write_all(&index_html.into_bytes()).unwrap()
}
