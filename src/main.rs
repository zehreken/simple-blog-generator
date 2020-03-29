use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};
use tiny_http::{Response, Server};

fn main() {
    // let test_file = fs::read_to_string("posts/2009-12-15-hello-again.markdown").unwrap();
    // let post: Post = toml::from_str(
    //     r#"
    //     [metadata]
    //     layout = 'post'
    //     title = 'Hello Again!'
    //     [content]
    //     markdown = '''test
    //     test'''
    //     "#,
    // )
    // .unwrap();
    // let post: Post = toml::from_str(test_file.as_str()).unwrap();

    let head_string = fs::read_to_string("head.html");
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
    entries.reverse();

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

    let mut index_markdown = String::new();
    for entry in entries {
        let path = entry;
        // This filters .DS_Store
        if let Some(_) = path.extension() {
            let contents =
                fs::read_to_string(path.clone()).expect("Something went wrong reading the file");

            let post: Post = toml::from_str(contents.as_str()).unwrap();

            let mut html_output = head_string.clone();
            html_output = html_output.replace("$title", post.title.as_str());
            html_output = html_output.replace("$date", post.date.as_str());
            html_output =
                html_output.replace("$content", to_markdown(post.markdown.as_str()).as_str());

            let file_name = path.file_stem().unwrap();
            let mut out_path = PathBuf::from("site");

            index_markdown.push('[');
            index_markdown.push_str(post.title.as_str());
            index_markdown.push(']');
            index_markdown.push('(');
            index_markdown.push_str(file_name.to_str().unwrap());
            index_markdown.push_str(".html");
            index_markdown.push_str(")<br>");

            out_path.push(file_name);
            out_path.set_extension("html");

            println!("{:?}", out_path.clone());
            let mut file = fs::File::create(out_path).expect("Error creating file");
            file.write_all(&html_output.into_bytes()[..])
                .expect("Error writing to file");
        }
    }

    let mut index_html = head_string.clone();
    index_html = index_html.replace("$title", "");
    index_html = index_html.replace("$date", "");
    index_html = index_html.replace("$content", to_markdown(index_markdown.as_str()).as_str());
    let mut index_file = fs::File::create("site/index.html").unwrap();
    index_file.write_all(&index_html.into_bytes()).unwrap();

    // Open browser, this works but running this in another thread after making sure
    // that serve is started is a better idea, this only works on MacOs
    std::process::Command::new("open")
        .arg(String::from("http://127.0.0.1:4000/index.html"))
        .output();

    let server = Server::http("127.0.0.1:4000").unwrap();

    for request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let file = fs::File::open(format!("site{}", request.url())).unwrap();
        let response = Response::from_file(file);
        request.respond(response);
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

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct Post {
    layout: String,
    title: String,
    date: String,
    markdown: String,
}
