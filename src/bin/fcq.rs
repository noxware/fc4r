use fileclass::core::{config::Config, document::Document, query};
use std::env;
use std::io;

// TODO: Handle errors here.
fn main() {
    let config = Config::load("fileclass/labels.toml").expect("Can't load config");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: fileclass <query>");
    }

    let prompt = &args[1..].join(" ");

    io::stdin()
        .lines()
        .map(|l| l.expect("Can't read line from stdio"))
        .for_each(|l| {
            let mut document = Document::from_filename(&l);
            document.expand(&config.labels);

            if query::check(&document, prompt) {
                println!("{}", l);
            }
        });
}
