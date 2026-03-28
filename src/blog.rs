use crate::{context::BuildContext, utils};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
pub struct Post {
    pub layout: String,
    pub title: String,
    pub created: String,
    #[allow(dead_code)]
    pub updated: String,
    pub tags: String,
    pub markdown: String,
}

impl Post {
    pub fn new(content: &str) -> Self {
        toml::from_str(content).expect("Error deserializing post content")
    }
}

fn build_tag_index(tag: &str, posts: &[String], ctx: &BuildContext) {
    let head_string = &ctx.templates.tag_head;

    let mut index_markdown = String::new();
    for post in posts {
        index_markdown.push_str(&format!("[$■ {}](../{}.html)  \r", post, post));
    }

    let mut index_html = head_string.clone();
    index_html = index_html.replace("$title", "");
    index_html = index_html.replace("$date", "");
    index_html = index_html.replace("$tags", "");
    index_html = index_html.replace("$content", &utils::to_html(&index_markdown));
    index_html = index_html.replace("$■", "<span>\u{2009}■\u{2009}</span>");

    let tags_dir = ctx.output_dir.join("tags");
    if !tags_dir.exists() {
        fs::create_dir(&tags_dir).expect("Failed to create tags directory");
    }

    let tag_file = tags_dir.join(format!("{}.html", tag.replace('#', "")));
    let mut index_file = fs::File::create(tag_file).expect("Failed to create tag index file");
    index_file
        .write_all(index_html.as_bytes())
        .expect("Failed to write tag index");
}

fn get_tags_link(tags: &str) -> String {
    let mut tags_link = String::from("<p>");
    for tag in tags.split(' ') {
        tags_link.push_str(&format!(
            "<a href=tags/{}.html>{}</a> ",
            tag.replace('#', ""),
            tag
        ));
    }
    tags_link.push_str("</p>");
    tags_link
}

pub fn build(ctx: &BuildContext) {
    if !ctx.output_dir.exists() {
        fs::create_dir_all(&ctx.output_dir).expect("Failed to create output directory");
    }

    let head_string = &ctx.templates.head;

    let mut entries = fs::read_dir(&ctx.posts_dir)
        .unwrap_or_else(|_| panic!("Directory {:?} not found", ctx.posts_dir))
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Error collecting entries");

    entries.sort();
    entries.reverse();

    fs::write(ctx.output_dir.join("main.css"), &ctx.templates.main_css)
        .expect("Failed to write main.css");

    let mut index_markdown = String::new();
    let mut prev_year = String::new();
    let mut posts_by_tag: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries {
        let path = entry;
        if path.extension().is_none() {
            continue;
        }

        let contents =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Error reading file {:?}", path));

        let post = Post::new(&contents);

        let mut html_output = head_string.clone();
        html_output = html_output.replace("$title", &post.title);
        html_output = html_output.replace("$date", &post.created);

        if post.layout == "post" {
            html_output = html_output.replace("$tags", &get_tags_link(&post.tags));
        } else {
            html_output = html_output.replace("$tags", "");
        }

        html_output = html_output.replace("$content", &utils::to_html(&post.markdown));

        let file_name = path.file_stem().unwrap();
        let mut out_path: PathBuf = ctx.output_dir.clone();

        if post.layout == "post" {
            let year = post.created.split('-').next().unwrap().to_owned();
            for tag in post.tags.split(' ') {
                let stem = file_name.to_str().unwrap().to_owned();
                match posts_by_tag.get_mut(tag) {
                    Some(posts) => posts.push(stem),
                    None => {
                        posts_by_tag.insert(tag.to_owned(), vec![stem]);
                    }
                }
            }
            if prev_year != year {
                index_markdown.push_str(&format!("#### {}  \r", year));
                prev_year = year;
            }
            index_markdown.push_str(&format!(
                "[$■ {}]({}.html)  \r",
                post.title,
                file_name.to_str().unwrap()
            ));
        }

        out_path.push(file_name);
        out_path.set_extension("html");

        println!("{:?}", &out_path);
        let mut file = fs::File::create(out_path).expect("Error creating file");
        file.write_all(html_output.as_bytes())
            .expect("Error writing to file");
    }

    for (tag, posts) in &posts_by_tag {
        println!("[TAG] {}", tag);
        build_tag_index(tag, posts, ctx);
    }

    let mut index_html = head_string.clone();
    index_html = index_html.replace("$title", "");
    index_html = index_html.replace("$date", "");
    index_html = index_html.replace("$tags", "");
    index_html = index_html.replace("$content", &utils::to_html(&index_markdown));
    index_html = index_html.replace("$■", "<span>\u{2009}■\u{2009}</span>");

    let mut index_file =
        fs::File::create(ctx.output_dir.join("index.html")).expect("Failed to create index.html");
    index_file
        .write_all(index_html.as_bytes())
        .expect("Failed to write index.html");
}
