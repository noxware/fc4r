use fileclass::core::{
    config::Config,
    document::Document,
    query::{check, CheckParams},
};
use std::env;
use std::io;

// TODO: Handle errors here.
fn main() {
    let config = Config::std_load().expect("Can't load config");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: fileclass <query>");
    }

    let prompt = &args[1..].join(" ");

    // TODO: Move into `aux` as function `read_stdin_documents` or `stdin_filenames_into_documents`.
    let result = io::stdin()
        .lines()
        .map(|l| l.expect("Can't read line from stdio"))
        .map(|l| Document::from_filename(&l))
        .filter(|d| {
            let params = CheckParams {
                prompt: &prompt,
                document: &d,
                library: &config.labels,
            };

            check(&params)
        });

    result.for_each(|d| println!("{}", d.filename));
}
