mod blog;
mod config;
mod context;
mod templates;
mod utils;
mod zettelkasten;

use context::BuildContext;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().expect("Cannot determine current directory");

    let project_root = flag_value(&args, "--root")
        .map(|r| cwd.join(r))
        .unwrap_or(cwd);

    let output = flag_value(&args, "--output").map(PathBuf::from);

    let config = config::load(&project_root).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let ctx = BuildContext::new(&project_root, config, output);
    zettelkasten::build(&ctx);
    blog::build(&ctx);
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}
