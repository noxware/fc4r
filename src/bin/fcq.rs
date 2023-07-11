use fileclass::core::{
    config::Config,
    query::{check, CheckParams},
};

use fileclass::extra::input::read_stdin_documents;

use std::env;

// TODO: Handle errors here.
fn main() {
    let config = Config::std_load().expect("Can't load config");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: fileclass <query>");
    }

    let prompt = &args[1..].join(" ");

    let result = read_stdin_documents().filter(|d| {
        let params = CheckParams {
            prompt: &prompt,
            document: &d,
            library: &config.labels,
        };

        check(&params)
    });

    result.for_each(|d| println!("{}", d.path));
}
