use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, io};

fn main() {
    let mut entries = fs::read_dir("posts")
        .expect("'posts' directory is not found")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("");

    entries.sort();

    for entry in entries {
        let mut path = entry;
        // let filename = "2018-11-22-collections-in-dotnet.markdown";
        if let Some(_) = path.extension() {
            let contents =
                fs::read_to_string(path.clone()).expect("Something went wrong reading the file");
            // println!("Markdown text:\n{}", contents);
            // let filestring = path.into_os_string().into_string().unwrap();
            // let a: Vec<&str> = filestring.split('.').collect();
            test_markdown(contents.as_str(), &mut path);
        }
    }
}

fn test_markdown(markdown: &str, path: &mut PathBuf) {
    use pulldown_cmark::{html, Options, Parser};

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    // println!("{}", html_output);

    let r = fs::create_dir("site");
    let r = match r {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error)
        },
    };

    let file_name = path.file_stem().unwrap();
    let mut out_path = PathBuf::from("site");
    out_path.push(file_name);
    out_path.set_extension("html");

    println!("{:?}", out_path.clone());
    let mut file = fs::File::create(out_path).expect("Error creating file");
    file.write_all(&html_output.into_bytes()[..])
        .expect("Error writing to file");

    // Check that the output is what we expected.
    let expected_html =
        "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    // assert_eq!(expected_html, &html_output);
}
