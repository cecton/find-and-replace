extern crate clap;
extern crate glob;
extern crate regex;

use std::fs;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = clap::App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            clap::Arg::with_name("glob")
                .required(true)
                .help("Glob (Unix shell style pattern)"),
        ).arg(
            clap::Arg::with_name("regex")
                .required(true)
                .help("Regular expression to search"),
        ).arg(
            clap::Arg::with_name("replacement")
                .required(true)
                .help("Replacement expression"),
        ).get_matches();

    let re = regex::Regex::new(matches.value_of("regex").unwrap())
        .expect("Could not compile regular expression");
    let replace = matches.value_of("replacement").unwrap();
    for entry in glob::glob(matches.value_of("glob").unwrap()).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => match fs::read_to_string(&path) {
                Ok(content) => {
                    let after = re.replace_all(&content, replace).into_owned();

                    if content != after {
                        match fs::write(&path, after) {
                            Ok(()) => println!("{}", path.display()),
                            Err(e) => eprintln!("{}: {}", path.display(), e),
                        }
                    }
                }
                Err(e) => eprintln!("{}: {}", path.display(), e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
