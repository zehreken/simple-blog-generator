use std::fs;
use std::io::prelude::*;

fn main() {
    let filename = "2018-04-08-struct-vs-class-dotnet.markdown";
    // let filename = "2018-11-22-collections-in-dotnet.markdown";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Markdown text:\n{}", contents);
    test_markdown(contents.as_str());
}

fn test_markdown(markdown: &str) {
    use pulldown_cmark::{html, Options, Parser};

    let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    println!("{}", html_output);

    let mut file = fs::File::create("index.html").expect("Error creating file");
    file.write_all(&html_output.into_bytes()[..])
        .expect("Error writing to file");

    // Check that the output is what we expected.
    let expected_html =
        "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    // assert_eq!(expected_html, &html_output);
}
