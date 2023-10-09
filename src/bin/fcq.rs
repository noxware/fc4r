use fileclass::{
    core::{
        label::LabelLibrary,
        query::{check, CheckParams},
    },
    extra::{input::read_stdin_messages, ipc::Message},
};

use std::env;

// TODO: Handle errors here.
fn main() {
    let mut library = LabelLibrary::empty();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: fileclass <query>");
    }

    let prompt = &args[1..].join(" ");

    for msg in read_stdin_messages() {
        match msg {
            Message::Config(c) => {
                library = c.labels;
            }
            Message::Document(d) => {
                let params = CheckParams {
                    prompt: &prompt,
                    document: &d,
                    library: &library,
                };

                if check(&params) {
                    println!("{}", d.path);
                }
            }
            _ => panic!("Unexpected message"),
        }
    }
}
